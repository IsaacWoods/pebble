mod validation;

use crate::{
    object::{
        address_space::AddressSpace,
        channel::{ChannelEnd, Message},
        memory_object::MemoryObject,
        task::{Task, TaskState},
        KernelObject,
    },
    per_cpu::PerCpu,
    Platform,
};
use alloc::{collections::BTreeMap, string::String, sync::Arc};
use bit_field::BitField;
use core::convert::TryFrom;
use hal::memory::{Flags, VirtualAddress};
use libpebble::{
    caps::Capability,
    syscall::{
        self,
        result::{handle_to_syscall_repr, status_to_syscall_repr, status_with_payload_to_syscall_repr},
        CreateMemoryObjectError,
        EarlyLogError,
        FramebufferInfo,
        GetFramebufferError,
        GetMessageError,
        MapMemoryObjectError,
        RegisterServiceError,
        SendMessageError,
        SubscribeToServiceError,
        CHANNEL_MAX_NUM_HANDLES,
    },
    Handle,
    ZERO_HANDLE,
};
use log::{info, trace, warn};
use spin::Mutex;
use validation::{UserPointer, UserSlice, UserString};

/// Maps the name of a service to the channel used to register new service users.
static SERVICE_MAP: Mutex<BTreeMap<String, Arc<ChannelEnd>>> = Mutex::new(BTreeMap::new());

/// This is the architecture-independent syscall handler. It should be called by the handler that
/// receives the syscall (each architecture is free to do this however it wishes). The only
/// parameter that is guaranteed to be valid is `number`; the meaning of the rest may be undefined
/// depending on how many parameters the specific system call takes.
pub fn handle_syscall<P>(number: usize, a: usize, b: usize, c: usize, d: usize, e: usize) -> usize
where
    P: Platform,
{
    info!("Syscall! number = {}, a = {}, b = {}, c = {}, d = {}, e = {}", number, a, b, c, d, e);
    let task = P::per_cpu().scheduler().get_mut().running_task.as_ref().unwrap();

    match number {
        syscall::SYSCALL_YIELD => yield_syscall::<P>(),
        syscall::SYSCALL_EARLY_LOG => status_to_syscall_repr(early_log(task, a, b)),
        syscall::SYSCALL_GET_FRAMEBUFFER => handle_to_syscall_repr(get_framebuffer(task, a)),
        syscall::SYSCALL_CREATE_MEMORY_OBJECT => handle_to_syscall_repr(create_memory_object(task, a, b, c)),
        syscall::SYSCALL_MAP_MEMORY_OBJECT => status_to_syscall_repr(map_memory_object(task, a, b, c)),
        syscall::SYSCALL_CREATE_CHANNEL => unimplemented!(),
        syscall::SYSCALL_SEND_MESSAGE => status_to_syscall_repr(send_message(task, a, b, c, d, e)),
        syscall::SYSCALL_GET_MESSAGE => status_with_payload_to_syscall_repr(get_message(task, a, b, c, d, e)),
        syscall::SYSCALL_WAIT_FOR_MESSAGE => unimplemented!(),
        syscall::SYSCALL_REGISTER_SERVICE => handle_to_syscall_repr(register_service(task, a, b)),
        syscall::SYSCALL_SUBSCRIBE_TO_SERVICE => handle_to_syscall_repr(subscribe_to_service(task, a, b)),

        _ => {
            warn!("Process made system call with invalid syscall number: {}", number);
            usize::MAX
        }
    }
}

fn yield_syscall<P>() -> usize
where
    P: Platform,
{
    info!("Process yielded!");
    P::per_cpu().scheduler().switch_to_next(TaskState::Ready);
    0
}

fn early_log<P>(task: &Arc<Task<P>>, str_length: usize, str_address: usize) -> Result<(), EarlyLogError>
where
    P: Platform,
{
    // Check the current task has the `EarlyLogging` capability
    if !task.capabilities.contains(&Capability::EarlyLogging) {
        return Err(EarlyLogError::TaskDoesNotHaveCorrectCapability);
    }

    // Check if the message is too long
    if str_length > 1024 {
        return Err(EarlyLogError::MessageTooLong);
    }

    // Check the message is valid UTF-8
    let message = UserString::new(str_address as *mut u8, str_length)
        .validate()
        .map_err(|_| EarlyLogError::MessageNotValidUtf8)?;

    trace!("Early log message from {}: {}", task.name, message);
    Ok(())
}

fn get_framebuffer<P>(task: &Arc<Task<P>>, info_address: usize) -> Result<Handle, GetFramebufferError>
where
    P: Platform,
{
    /*
     * Check that the task has the correct capability.
     */
    if !task.capabilities.contains(&Capability::GetFramebuffer) {
        return Err(GetFramebufferError::AccessDenied);
    }

    let (info, memory_object) = crate::FRAMEBUFFER.try_get().ok_or(GetFramebufferError::NoFramebufferCreated)?;
    let handle = task.add_handle(memory_object.clone());

    UserPointer::new(info_address as *mut FramebufferInfo, true)
        .write(*info)
        .map_err(|()| GetFramebufferError::InfoAddressIsInvalid)?;

    Ok(handle)
}

fn create_memory_object<P>(
    task: &Arc<Task<P>>,
    virtual_address: usize,
    size: usize,
    flags: usize,
) -> Result<Handle, CreateMemoryObjectError>
where
    P: Platform,
{
    let writable = flags.get_bit(0);
    let executable = flags.get_bit(1);

    // TODO: do something more sensible with this when we have a concept of physical memory "ownership"
    let physical_start = crate::PHYSICAL_MEMORY_MANAGER.get().alloc_bytes(size);

    let memory_object = MemoryObject::new(
        task.id(),
        VirtualAddress::new(virtual_address),
        physical_start,
        size,
        Flags { writable, executable, user_accessible: true, ..Default::default() },
    );

    Ok(task.add_handle(memory_object))
}

fn map_memory_object<P>(
    task: &Arc<Task<P>>,
    memory_object_handle: usize,
    address_space_handle: usize,
    address_ptr: usize,
) -> Result<(), MapMemoryObjectError>
where
    P: Platform,
{
    let memory_object_handle =
        Handle::try_from(memory_object_handle).map_err(|_| MapMemoryObjectError::InvalidHandle)?;
    let address_space_handle =
        Handle::try_from(address_space_handle).map_err(|_| MapMemoryObjectError::InvalidHandle)?;

    let memory_object = task
        .handles
        .read()
        .get(&memory_object_handle)
        .ok_or(MapMemoryObjectError::InvalidHandle)?
        .clone()
        .downcast_arc::<MemoryObject>()
        .ok()
        .ok_or(MapMemoryObjectError::NotAMemoryObject)?;

    if address_space_handle == ZERO_HANDLE {
        /*
         * If the AddressSpace handle is the zero handle, we map the MemoryObject into the calling task's
         * address space.
         */
        task.address_space.map_memory_object(memory_object.clone(), &crate::PHYSICAL_MEMORY_MANAGER.get())?;
    } else {
        task.handles
            .read()
            .get(&memory_object_handle)
            .ok_or(MapMemoryObjectError::InvalidHandle)?
            .clone()
            .downcast_arc::<AddressSpace<P>>()
            .ok()
            .ok_or(MapMemoryObjectError::NotAnAddressSpace)?
            .map_memory_object(memory_object.clone(), &crate::PHYSICAL_MEMORY_MANAGER.get())?;
    }

    /*
     * An address pointer of `0` signals to the kernel that the caller does not need to know the virtual
     * address, so don't bother writing it back.
     */
    if address_ptr != 0x0 {
        let mut address_ptr = UserPointer::new(address_ptr as *mut VirtualAddress, true);
        address_ptr
            .write(memory_object.virtual_address)
            .map_err(|()| MapMemoryObjectError::AddressPointerInvalid)?;
    }

    Ok(())
}

fn send_message<P>(
    task: &Arc<Task<P>>,
    channel_handle: usize,
    byte_address: usize,
    num_bytes: usize,
    handles_address: usize,
    num_handles: usize,
) -> Result<(), SendMessageError>
where
    P: Platform,
{
    use libpebble::syscall::CHANNEL_MAX_NUM_BYTES;

    if num_bytes > CHANNEL_MAX_NUM_BYTES {
        return Err(SendMessageError::TooManyBytes);
    }
    if num_handles > CHANNEL_MAX_NUM_HANDLES {
        return Err(SendMessageError::TooManyHandles);
    }

    let channel_handle = Handle::try_from(channel_handle).map_err(|_| SendMessageError::InvalidChannelHandle)?;
    let bytes = if num_bytes == 0 {
        &[]
    } else {
        UserSlice::new(byte_address as *mut u8, num_bytes)
            .validate_read()
            .map_err(|()| SendMessageError::BytesAddressInvalid)?
    };
    let handles = if num_handles == 0 {
        &[]
    } else {
        UserSlice::new(handles_address as *mut Handle, num_handles)
            .validate_read()
            .map_err(|()| SendMessageError::HandlesAddressInvalid)?
    };
    let handle_objects = {
        let mut arr = [None; CHANNEL_MAX_NUM_HANDLES];
        for (i, handle) in handles.iter().enumerate() {
            arr[i] = match task.handles.read().get(handle) {
                Some(object) => Some(object.clone()),
                None => return Err(SendMessageError::InvalidTransferredHandle),
            };

            /*
             * We're transferring the handle's object, so we remove the handle to it from the sending task.
             */
            task.handles.write().remove(&handle);
        }
        arr
    };
    trace!("Message sent down channel: {:x?} ({} handles transferred)", bytes, handles.len());

    task.handles
        .read()
        .get(&channel_handle)
        .ok_or(SendMessageError::InvalidChannelHandle)?
        .clone()
        .downcast_arc::<ChannelEnd>()
        .ok()
        .ok_or(SendMessageError::NotAChannel)?
        .send(Message { bytes: bytes.to_vec(), handle_objects })
}

fn get_message<P>(
    task: &Arc<Task<P>>,
    channel_handle: usize,
    bytes_address: usize,
    bytes_len: usize,
    handles_address: usize,
    handles_len: usize,
) -> Result<usize, GetMessageError>
where
    P: Platform,
{
    let channel_handle = Handle::try_from(channel_handle).map_err(|_| GetMessageError::InvalidChannelHandle)?;

    let channel = task
        .handles
        .read()
        .get(&channel_handle)
        .ok_or(GetMessageError::InvalidChannelHandle)?
        .clone()
        .downcast_arc::<ChannelEnd>()
        .ok()
        .ok_or(GetMessageError::NotAChannel)?;

    channel.receive(|message| {
        let num_handles = message.num_handles();

        if message.bytes.len() > bytes_len {
            return Err((message, GetMessageError::BytesBufferTooSmall));
        }
        if num_handles > handles_len {
            return Err((message, GetMessageError::HandlesBufferTooSmall));
        }

        let byte_buffer = match UserSlice::new(bytes_address as *mut u8, message.bytes.len()).validate_write() {
            Ok(buffer) => buffer,
            Err(()) => return Err((message, GetMessageError::BytesAddressInvalid)),
        };
        let handles_buffer = match UserSlice::new(handles_address as *mut Handle, num_handles).validate_write() {
            Ok(buffer) => buffer,
            Err(()) => return Err((message, GetMessageError::HandlesAddressInvalid)),
        };

        byte_buffer.copy_from_slice(&message.bytes);
        for i in 0..num_handles {
            handles_buffer[i] = task.add_handle(message.handle_objects[i].as_ref().unwrap().clone());
        }

        let mut status = 0;
        status.set_bits(16..32, message.bytes.len());
        status.set_bits(32..48, num_handles);
        Ok(status)
    })
}

fn register_service<P>(
    task: &Arc<Task<P>>,
    name_length: usize,
    name_ptr: usize,
) -> Result<Handle, RegisterServiceError>
where
    P: Platform,
{
    use libpebble::syscall::SERVICE_NAME_MAX_LENGTH;

    // Check that the task has the `ServiceProvider` capability
    if !task.capabilities.contains(&Capability::ServiceProvider) {
        return Err(RegisterServiceError::TaskDoesNotHaveCorrectCapability);
    }

    // Check that the name is not too short or long
    if name_length == 0 || name_length > SERVICE_NAME_MAX_LENGTH {
        return Err(RegisterServiceError::NameLengthNotValid);
    }

    let service_name = UserString::new(name_ptr as *mut u8, name_length)
        .validate()
        .map_err(|()| RegisterServiceError::NamePointerNotValid)?;

    info!("Task {} has registered a service called {}", task.name, service_name);
    let channel = ChannelEnd::new_kernel_channel(task.id());
    SERVICE_MAP.lock().insert(task.name.clone() + "." + service_name, channel.clone());

    Ok(task.add_handle(channel))
}

fn subscribe_to_service<P>(
    task: &Arc<Task<P>>,
    name_length: usize,
    name_ptr: usize,
) -> Result<Handle, SubscribeToServiceError>
where
    P: Platform,
{
    use libpebble::syscall::SERVICE_NAME_MAX_LENGTH;

    // Check that the task has the `ServiceUser` capability
    if !task.capabilities.contains(&Capability::ServiceUser) {
        return Err(SubscribeToServiceError::TaskDoesNotHaveCorrectCapability);
    }

    // Check that the name is not too short or long
    if name_length == 0 || name_length > SERVICE_NAME_MAX_LENGTH {
        return Err(SubscribeToServiceError::NameLengthNotValid);
    }

    let service_name = UserString::new(name_ptr as *mut u8, name_length)
        .validate()
        .map_err(|()| SubscribeToServiceError::NamePointerNotValid)?;

    if let Some(register_channel) = SERVICE_MAP.lock().get(service_name) {
        // Create new channel to allow the two tasks to communicate
        let (provider_end, user_end) = ChannelEnd::new_channel(task.id());

        // TODO: send a message down `register_channel` telling it about `provider_end`

        // Return the user's end of the new channel to it
        Ok(task.add_handle(user_end))
    } else {
        Err(SubscribeToServiceError::NoServiceWithThatName)
    }
}
