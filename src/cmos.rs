pub static mut SECOND: u8 = 0u8;

#[repr(u16)]
enum CmosAddresses {
    Address = 0x70,
    Data = 0x71,
}

pub unsafe fn read_sec() {
    ::portio::out_byte(CmosAddresses::Address as u16, 0x00);
    SECOND = ::portio::in_byte(CmosAddresses::Data as u16);

    ::portio::out_byte(CmosAddresses::Address as u16, 0x0B);
    if ::portio::in_byte(CmosAddresses::Data as u16) & 0x04 != 0 {
        SECOND = (SECOND & 0x0F) + ((SECOND / 16) * 10);
    }
}
