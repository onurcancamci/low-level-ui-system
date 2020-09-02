pub fn to_u32(buf: &[u8; 4]) -> u32 {
    let mut val = [0u8; 4];
    val.clone_from_slice(buf);
    u32::from_le_bytes(val)
}

pub fn from_u32(val: u32) -> [u8; 4] {
    val.to_le_bytes()
}
