#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn kentry() -> ! {
    use core::fmt::Write;

    let uart = unsafe { &mut *(0x10000000 as *mut hal_riscv::hw::uart16550::Uart16550) };
    writeln!(uart, "Hello from the kernel!").unwrap();
    loop {}
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    let uart = unsafe { &mut *(0x10000000 as *mut hal_riscv::hw::uart16550::Uart16550) };
    write!(uart, "Panic :(").unwrap();
    loop {}
}
