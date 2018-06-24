#![no_std]
#![no_main]
#![feature(panic_implementation, abi_x86_interrupt, asm)]

#[macro_use]
extern crate lazy_static;
extern crate volatile;
extern crate spin;
extern crate x86_64;

use core::panic::PanicInfo;

#[macro_use]
pub mod vga;
pub use vga::STD_OUT;

pub mod cmos;
pub mod portio;

mod cpu_exceptions;
mod bee_movie;

pub const VERSION: &str = "1.0.0";

fn smallest(first: usize, second: usize) -> usize {
    if first > second {
        second
    } else {
        first
    }
}

pub extern "C" fn start() -> ! {
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
        let write = &bee_movie::SCRIPT.as_bytes()[read..smallest(movie_len, read + vga::BUFFER_WIDTH)];
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

/// (Used by linker) Linux start
#[no_mangle]
pub extern "C" fn _start() {
    start();
}

/// (Used by linker) macOS & Windows start
#[no_mangle]
pub extern "C" fn main() {
    start();
}

/// (Used by linker) Windows start
#[no_mangle]
pub extern "C" fn mainCRTStartup() {
    start();
}

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("{}", info);
    loop {}
}
