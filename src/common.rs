
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

pub fn code_from(c: u8, dd: u8) -> u8 {
        ((c & 0xF7) << 5) | (dd & 0x1F)
}

pub fn to_code_str(code: u8) -> String{
    let p = code >> 5;
    let s = code & 0x1F;
    format!("{}.{:02}", p, s)
}

#[cfg(test)]
mod test {
    use crate::common::code_from;


    #[test]
    fn test_code_from() {
        let i = code_from(4, 00);
        assert_eq!(code_from(2, 01), 0x41);
        assert_eq!(code_from(4, 04), 0x84);
        assert_ne!(code_from(5, 0), 0x50)
    }
}