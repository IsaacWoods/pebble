#![no_std]
#![no_main]

use core::fmt::Write;
use volatile::Volatile;

core::arch::global_asm!(
    "
    .section .text.start
    .global _start
    _start:
        // Zero the BSS
        la t0, _bss_start
        la t1, _bss_end
        bgeu t0, t1, .bss_zero_loop_end
    .bss_zero_loop:
        sd zero, (t0)
        addi t0, t0, 8
        bltu t0, t1, .bss_zero_loop
    .bss_zero_loop_end:

        la sp, _stack_top

        jal main
        unimp
"
);

#[no_mangle]
pub fn main() -> ! {
    let serial = unsafe { &mut *(0x0250_0000 as *mut Uart) };
    writeln!(serial, "Poplar's boot0 is running!").unwrap();

    loop {}
}

#[repr(C)]
pub struct Uart {
    data: Volatile<u32>,
    interrupt_enable: Volatile<u32>,
    interrupt_identity: Volatile<u32>,
    line_control: Volatile<u32>,
    modem_control: Volatile<u32>,
    line_status: Volatile<u32>,
    modem_status: Volatile<u32>,
    scratch: Volatile<u32>,
}

impl Uart {
    fn line_status(&self) -> u32 {
        self.line_status.read()
    }

    pub fn write(&self, data: u8) {
        while (self.line_status() & 0x20) == 0 {}
        self.data.write(data as u32);
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' => {
                    self.write(b'\n');
                    self.write(b'\r');
                }
                other => self.write(other),
            }
        }
        Ok(())
    }
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    let serial = unsafe { &mut *(0x0250_0000 as *mut Uart) };
    let _ = write!(serial, "boot0: PANIC!");
    // if let Some(message) = info.message() {
    //     if let Some(location) = info.location() {
    //         let _ = writeln!(
    //             LOGGER.serial.lock(),
    //             "PANIC: {} ({} - {}:{})",
    //             message,
    //             location.file(),
    //             location.line(),
    //             location.column()
    //         );
    //     } else {
    //         let _ = writeln!(LOGGER.serial.lock(), "PANIC: {} (no location info)", message);
    //     }
    // }
    loop {}
}
