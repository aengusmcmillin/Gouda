pub fn u16_from_bytes(bytes: [u8; 2]) -> u16 {
    ((bytes[0] as u16) << 8) + ((bytes[1] as u16) << 0)
}

pub fn u32_from_bytes(bytes: [u8; 4]) -> u32 {
    ((bytes[0] as u32) << 24)
        + ((bytes[1] as u32) << 16)
        + ((bytes[2] as u32) << 8)
        + ((bytes[3] as u32) << 0)
}

pub fn _i16_from_bytes(bytes: [u8; 2]) -> i16 {
    ((bytes[0] as i16) << 8) + ((bytes[1] as i16) << 0)
}

pub fn _i32_from_bytes(bytes: [u8; 4]) -> i32 {
    ((bytes[0] as i32) << 24)
        + ((bytes[1] as i32) << 16)
        + ((bytes[2] as i32) << 8)
        + ((bytes[3] as i32) << 0)
}
