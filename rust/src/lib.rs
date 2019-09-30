#![no_std]
#![feature(abi_x86_interrupt, asm)]

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;
extern crate x86_64;

use core::panic::PanicInfo;

#[macro_use]
pub mod vga;
pub use self::vga::STD_OUT;

pub mod cmos;
pub mod portio;

mod bee_movie;
mod cpu_exceptions;

pub const VERSION: &str = "1.0.0";

/// Get the smallest of two ordered values.
fn smallest<T: core::cmp::PartialOrd>(first: T, second: T) -> T {
    if first > second { second } else { first }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    self::cpu_exceptions::IDT.load();
    println!("beeOS version {}", VERSION);
    let mut writer = STD_OUT.lock();
    let mut read = 0;
    let movie_len = bee_movie::SCRIPT.len();
    loop {
        if read >= movie_len {
            read = 0;
            writer.clear_screen();
        }
        let write = &bee_movie::SCRIPT.as_bytes()
            [read..smallest(movie_len, read + vga::BUFFER_WIDTH)];
        for ref c in write.iter() {
            writer.write(**c);
        }
        writer.new_line();

        read += vga::BUFFER_WIDTH;
        let second;
        unsafe {
            cmos::read_sec();
            second = cmos::SECOND;
        }
        loop {
            let new_second;
            unsafe {
                cmos::read_sec();
                new_second = cmos::SECOND;
            }
            if new_second != second {
                break;
            }
        }
    }
}

#[panic_handler]
#[no_mangle]
#[allow(unreachable_code)]
pub fn panic(info: &PanicInfo) -> ! {
    // uh oh, the kernel panicked for any reason.
    // this must be handled in a proper way such that a developer can figure out
    // what to do.

    println!();
    println!(
        "beeOS has panicked. Please search this issue on GitHub, and report \
         if needed!\n{}",
        info
    );

    // now start looping
    loop {}

    // and asm version just to be entirely safe:
    unsafe {
        asm!(
            "loop:
            jmp loop
            hlt"
            :::: "volatile"
        );
    }

    // to handle the !
    loop {}
}
