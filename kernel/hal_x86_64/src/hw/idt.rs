use super::{registers::CpuFlags, DescriptorTablePointer};
use crate::hw::gdt::{PrivilegeLevel, SegmentSelector};
use bit_field::BitField;
use core::{
    mem,
    ops::{Index, IndexMut},
};
use hal::memory::VirtualAddress;

/// The type of a function that can be used as an interrupt handler. It's marked as diverging
/// because we don't exactly 'return' from an interrupt. This should not be used directly to create
/// interrupt handlers; instead, use the `wrap_handler` macros.
pub type HandlerFunc = extern "C" fn() -> !;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    address_0_15: u16,

    /// This is the segment selector of the code segment in the GDT that should be entered when
    /// this vector is handled. This is normally set to the kernel's code segment.
    segment_selector: u16,

    /// If this is not `0`, this is used as an index into the Interrupt Stack Table described in
    /// the currently active TSS. When this vector is handled, the address in that index of the
    /// IST will be loaded into `RSP`. This allows us to nicely handle kernel stack overflows.
    ist_index: u8,

    ///    7                           0
    ///  +---+---+---+---+---+---+---+---+
    ///  | P |  DPL  | 0 |    GateType   |
    ///  +---+---+---+---+---+---+---+---+
    ///
    ///  P = Present
    ///  DPL = Descriptor Privilege Level
    flags: u8,
    address_16_31: u16,
    address_32_63: u32,
    _reserved: u32,
}

impl IdtEntry {
    pub const fn missing() -> IdtEntry {
        IdtEntry {
            address_0_15: 0,
            segment_selector: 0,
            ist_index: 0,
            flags: 0b0_00_0_1110,
            address_16_31: 0,
            address_32_63: 0,
            _reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: HandlerFunc, code_selector: SegmentSelector) -> &mut Self {
        /*
         * Set the Present bit, and set the gate type to an Interrupt Gate.
         */
        let mut flags: u8 = 0;
        flags.set_bits(0..4, 0b1110);
        flags.set_bit(7, true);
        self.flags = flags;

        self.segment_selector = code_selector.table_offset();

        let address = handler as u64;
        self.address_0_15 = address.get_bits(0..16) as u16;
        self.address_16_31 = address.get_bits(16..32) as u16;
        self.address_32_63 = address.get_bits(32..64) as u32;

        self
    }

    pub fn set_ist_index(&mut self, stack_index: u8) -> &mut Self {
        self.ist_index = stack_index;
        self
    }

    pub fn set_privilege_level(&mut self, privilege_level: PrivilegeLevel) -> &mut Self {
        self.flags.set_bits(5..7, privilege_level as u8);
        self
    }
}

/// Represents the Interrupt Descriptor Table in memory. We align to 16, so the table starts on an 8-aligned
/// address to improve cache-line behaviour, and so each entry's fields are well-aligned.
#[repr(C)]
#[repr(align(16))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

macro getter($entry: expr, $name: ident) {
    #[allow(dead_code)]
    pub fn $name(&mut self) -> &mut IdtEntry {
        &mut self[$entry]
    }
}

impl Idt {
    pub const fn empty() -> Idt {
        Idt { entries: [IdtEntry::missing(); 256] }
    }

    getter!(0, divide_error);
    getter!(1, debug_exception);
    getter!(2, nmi);
    getter!(3, breakpoint);
    getter!(4, overflow);
    getter!(5, bound_range_exceeded);
    getter!(6, invalid_opcode);
    getter!(7, device_not_available);
    getter!(8, double_fault);
    // XXX: 9 is reserved (never generated by x86_64 processors)
    getter!(10, invalid_tss);
    getter!(11, segment_not_present);
    getter!(12, stack_segment_fault);
    getter!(13, general_protection_fault);
    getter!(14, page_fault);
    // XXX: 15 is reserved
    getter!(16, x87_fault);
    getter!(17, alignment_check);
    getter!(18, machine_check);
    getter!(19, simd_exception);
    getter!(20, virtualization_exception);
    // XXX: 21 through 31 are reserved

    pub fn load(&self) {
        let idt_ptr = DescriptorTablePointer {
            limit: mem::size_of::<Self>() as u16 - 1,
            base: VirtualAddress::from(self as *const _),
        };

        unsafe {
            asm!("lidt [{}]", in(reg) &idt_ptr);
        }
    }
}

impl Index<u8> for Idt {
    type Output = IdtEntry;

    fn index(&self, index: u8) -> &Self::Output {
        &self.entries[index as usize]
    }
}

impl IndexMut<u8> for Idt {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.entries[index as usize]
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,

    pub instruction_pointer: VirtualAddress,
    pub code_segment: u64,
    pub cpu_flags: CpuFlags,
    pub stack_pointer: VirtualAddress,
    pub stack_segment: u64,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ExceptionWithErrorStackFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,

    pub error_code: u64,
    pub instruction_pointer: VirtualAddress,
    pub code_segment: u64,
    pub cpu_flags: CpuFlags,
    pub stack_pointer: VirtualAddress,
    pub stack_segment: u64,
}

pub macro wrap_handler($name: path) {{
    #[naked]
    extern "C" fn wrapper() -> ! {
        unsafe {
            asm!("/*
                   * Save registers. We only need to save the scratch registers (rax, rcx, rdx, rdi, rsi, r8, r9,
                   * and r10) as Rust will handle callee-saved registers, but we save all of them so we can inspect
                   * register contents in a handler if we need to. Order must match `InterruptStackFrame` and
                   * `ExceptionWithErrorStackFrame`.
                   */
                  push rax
                  push rbx
                  push rcx
                  push rdx
                  push rsi
                  push rdi
                  push rbp
                  push r8
                  push r9
                  push r10
                  push r11
                  push r12
                  push r13
                  push r14
                  push r15

                  /*
                   * Without an error code, a total of `0xa0` bytes are pushed onto the stack. Because `rsp+8`
                   * needs to be divisible by `0x10`, we align the stack.
                   */
                  mov rdi, rsp
                  sub rsp, 8
                  call {}
                  add rsp, 8

                  pop r15
                  pop r14
                  pop r13
                  pop r12
                  pop r11
                  pop r10
                  pop r9
                  pop r8
                  pop rbp
                  pop rdi
                  pop rsi
                  pop rdx
                  pop rcx
                  pop rbx
                  pop rax

                  iretq",
                sym $name,
                options(noreturn)
            )
        }
    }

    wrapper
}}

pub macro wrap_handler_with_error_code($name: path) {{
    #[naked]
    extern "C" fn wrapper() -> ! {
        unsafe {
            asm!("push rax
                  push rbx
                  push rcx
                  push rdx
                  push rsi
                  push rdi
                  push rbp
                  push r8
                  push r9
                  push r10
                  push r11
                  push r12
                  push r13
                  push r14
                  push r15

                  /*
                   * With an error code, a total of `0xa8` bytes are pushed onto the stack, and so `rsp+8` is already
                   * aligned correctly.
                   */
                  mov rdi, rsp
                  call {}

                  pop r15
                  pop r14
                  pop r13
                  pop r12
                  pop r11
                  pop r10
                  pop r9
                  pop r8
                  pop rbp
                  pop rdi
                  pop rsi
                  pop rdx
                  pop rcx
                  pop rbx
                  pop rax

                  iretq",
                sym $name,
                options(noreturn)
            )
        }
    }

    wrapper
}}
