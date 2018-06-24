pub unsafe fn out_byte(port: u16, val: u8) {
    asm!("outb $1,$0" :: "{dx}" (port), "{ax}" (val) :: "volatile");
}

pub unsafe fn in_byte(port: u16) -> u8 {
    let ret: u8;
    asm!("inb $1, $0" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    ret
}
