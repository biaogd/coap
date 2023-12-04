
pub fn url_parse(uri: &str) {
    match url::Url::parse(uri) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

trait Uri {
    
    fn parse(&self, uri: &str);
}

pub fn u16_to_bytes(value: u16) -> Vec<u8> {
    let first_byte = ((value >> 4) & 0xFF) as u8;
    let second_byte = (value & 0xFF) as u8;
    vec![first_byte, second_byte]
}

pub fn u32_to_bytes(value: u32) -> Vec<u8> {
    let byte1 = ((value >> 24) & 0xFF) as u8;
    let byte2 = ((value >> 16) & 0xFF) as u8;
    let byte3 = ((value >> 8) & 0xFF) as u8;
    let byte4 = (value & 0xFF) as u8;
    vec![byte1, byte2, byte3, byte3, byte4]
}