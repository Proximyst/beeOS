use crate::portio::{
    in_byte,
    out_byte,
};

// TODO: Replace this by `read_sec` reading and returning.
/// The current stored read.
pub static mut SECOND: u8 = 0u8;

/// The addresses to the CMOS for use in `inb` and `outb`.
#[derive(Clone, Copy)]
#[repr(u16)]
enum CmosAddresses {
    /// The main address to the CMOS.
    Address = 0x70,

    /// The data return part of the CMOS.
    Data = 0x71,
}

/// Read the current second of the minute and store it in `SECOND`.
pub unsafe fn read_sec() {
    out_byte(CmosAddresses::Address as u16, 0x00);
    SECOND = in_byte(CmosAddresses::Data as u16);

    out_byte(CmosAddresses::Address as u16, 0x0B);
    if in_byte(CmosAddresses::Data as u16) & 0x04 != 0 {
        SECOND = (SECOND & 0x0F) + ((SECOND / 16) * 10);
    }
}
