pub fn convert_i8_to_u8vec(value: i8) -> Vec<u8> {
    vec![value as u8]
}

pub fn convert_i16_to_u8vec(value: i16) -> Vec<u8> {
    vec![(value as u16 & 0x00FF) as u8, (value as u16 & 0xFF00 >> 8) as u8]
}

pub fn convert_i32_to_u8vec(value: i32) -> Vec<u8> {
    vec![(value as u32 & 0x000000FF) as u8,
         ((value as u32 & 0x0000FF00) >> 8) as u8,
         ((value as u32 & 0x00FF0000) >> 16) as u8,
         ((value as u32 & 0xFF000000) >> 24) as u8]
}

pub fn convert_i64_to_u8vec(value: i64) -> Vec<u8> {
    vec![(value as u64 & 0x00000000000000FF) as u8,
         ((value as u64 & 0x000000000000FF00) >> 8) as u8,
         ((value as u64 & 0x0000000000FF0000) >> 16) as u8,
         ((value as u64 & 0x00000000FF000000) >> 24) as u8,

         ((value as u64 & 0x000000FF00000000) >> 32) as u8,
         ((value as u64 & 0x0000FF0000000000) >> 40) as u8,
         ((value as u64 & 0x00FF000000000000) >> 48) as u8,
         ((value as u64 & 0xFF00000000000000) >> 56) as u8]
}
