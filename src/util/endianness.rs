pub fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

pub fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

pub fn as_i32_be(array: &[u8; 4]) -> i32 {
    ((array[0] as i32) << 24)
        + ((array[1] as i32) << 16)
        + ((array[2] as i32) << 8)
        + ((array[3] as i32) << 0)
}

pub fn as_i32_le(array: &[u8; 4]) -> i32 {
    ((array[0] as i32) << 0)
        + ((array[1] as i32) << 8)
        + ((array[2] as i32) << 16)
        + ((array[3] as i32) << 24)
}
