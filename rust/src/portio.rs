/// Write the byte given to the port by address.
///
/// This will only write and does not wait for a response.
/// Use `in_byte` to get a blocking response.
pub unsafe fn out_byte(port: u16, val: u8) {
    asm!(
        // use the x86_64 instruction `outb` to write a byte
        "outb $1, $0"
        : // no outputs
        : "{dx}" (port), "{ax}" (val) // input port from %DX and value from %AX
        : "memory" // get from memory
        : "volatile" // this has important side-effects! don't optimise it!
    );
}

/// Read a byte from the given port by address.
///
/// This will only read and waits for a response.
/// To write then read, use `out_byte` first, then this.
pub unsafe fn in_byte(port: u16) -> u8 {
    // The value to return.
    // This will be stored in %AX.
    let ret: u8;

    asm!(
        // use the x86_64 instruction `inb` to read a byte, blockingly
        "inb $1, $0"
        : "={ax}"(ret) // output to `ret` thru %AX
        : "{dx}"(port) // read from `port` thru %DX
        : "memory" // get from and store to memory
        : "volatile" // this has important side-effects! don't optimise it!
    );

    ret
}
