use core::convert::TryFrom;
use std::collections::BTreeMap;

use rand::Rng;

use crate::error::{InvalidContentFormat, InvalidType};

/// coap version
const VER: u8 = 1;
const TOKEN_LEN: u8 = 8;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    ver: u8,
    msg_type: u8,
    tkl: u8,
    code: u8,
    msg_id: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CoAPFrame {
    pub header: Header,
    token: Vec<u8>,
    options: BTreeMap<u16, Vec<Vec<u8>>>,
    ff: u8,
    payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Con,
    Non,
    Ack,
    Rst,
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        match value {
            MessageType::Con => 0,
            MessageType::Non => 1,
            MessageType::Ack => 2,
            MessageType::Rst => 3,
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = InvalidType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Con),
            1 => Ok(MessageType::Non),
            2 => Ok(MessageType::Ack),
            3 => Ok(MessageType::Rst),
            _ => Err(InvalidType)
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptionEnum {
    IfMatch,
    UriHost,
    ETag,
    IfNoneMatch,
    UriPort,
    LocationPath,
    UriPath,
    ContentFormat,
    MaxAge,
    UriQuery,
    Accept,
    LocationQuery,
    ProxyUri,
    ProxyScheme,
    Size1,
    Unknown(u16),
}

impl From<u16> for OptionEnum {
    fn from(value: u16) -> Self {
        match value {
            1 => OptionEnum::IfMatch,
            3 => OptionEnum::UriHost,
            4 => OptionEnum::ETag,
            5 => OptionEnum::IfNoneMatch,
            7 => OptionEnum::UriPort,
            8 => OptionEnum::LocationPath,
            11 => OptionEnum::UriPath,
            12 => OptionEnum::ContentFormat,
            14 => OptionEnum::MaxAge,
            15 => OptionEnum::UriQuery,
            17 => OptionEnum::Accept,
            20 => OptionEnum::LocationQuery,
            35 => OptionEnum::ProxyUri,
            39 => OptionEnum::ProxyScheme,
            60 => OptionEnum::Size1,
            _ => OptionEnum::Unknown(value),
        }
    }
}

impl From<OptionEnum> for u16 {
    fn from(value: OptionEnum) -> Self {
        match value {
            OptionEnum::IfMatch => 1,
            OptionEnum::UriHost => 3,
            OptionEnum::ETag => 4,
            OptionEnum::IfNoneMatch => 5,
            OptionEnum::UriPort => 7,
            OptionEnum::LocationPath => 8,
            OptionEnum::UriPath => 11,
            OptionEnum::ContentFormat => 12,
            OptionEnum::MaxAge => 14,
            OptionEnum::UriQuery => 15,
            OptionEnum::Accept => 17,
            OptionEnum::LocationQuery => 20,
            OptionEnum::ProxyUri => 35,
            OptionEnum::ProxyScheme => 39,
            OptionEnum::Size1 => 60,
            OptionEnum::Unknown(value) => value,
        }
    }
}

// impl From<OptionEnum> for String {
//     fn from(value: OptionEnum) -> Self {
//         match value {
//             OptionEnum::IfMatch => 
//         }
//     }
// }

/// CoAP
struct CoapOption {
    number: u16,
    value: Vec<u8>,
}

impl CoapOption {
    fn new(number: u16, value: Vec<u8>) -> CoapOption {
        CoapOption { number, value }
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded_option: Vec<u8> = Vec::new();

        let delta = if self.number < 13 {
            self.number as u8
        } else if self.number < 269 {
            13
        } else {
            14
        };

        let length = self.value.len();
        let vl = if length < 13 {
            length as u8
        } else if length < 269 {
            13
        } else {
            14
        };

        encoded_option.push((delta << 4) | vl);

        if delta == 13 {
            encoded_option.push((self.number - 13) as u8);
        }

        if delta == 14 {
            encoded_option.push((self.number - 269_u16).try_into().unwrap())
        }

        if vl == 13 {
            encoded_option.push((length - 13) as u8);
        }

        if vl == 14 {
            encoded_option.push(((length - 269) as u16).try_into().unwrap());
        }

        encoded_option.extend_from_slice(&self.value);

        encoded_option
    }

}


impl Header {

    pub fn new(msg_type: u8, code: u8) -> Self {
        Header {
            ver: VER,
            msg_type: msg_type,
            tkl: TOKEN_LEN,
            code,
            msg_id: generate_coap_message_id(),
        }
    }

    pub fn set_tkl(&mut self, tkl: u8) {
        self.tkl = tkl;
    }

    pub fn set_msg_id(&mut self, msg_id: u16) {
        self.msg_id = msg_id;
    }

    pub fn get_type(&self) -> u8 {
        self.msg_type
    }

    pub fn get_code(&self) -> u8 {
        self.code
    }

    fn to_bytes(&self) -> [u8; 4] {
        let t = self.ver << 6 | self.msg_type << 4 | self.tkl;
        let msg_buf = self.msg_id.to_be_bytes();
        [t, self.code, msg_buf[0], msg_buf[1]]
    }

    fn from_bytes(data: &[u8]) -> Option<Header> {
        if data.len() < 4 {
            return None;
        }

        let ver = data[0] >> 6;
        let msg_type = data[0] >> 4 & 0x3;
        let tkl = data[0] & 0xF;

        let code = data[1];
        let msg_id = u16::from_be_bytes([data[2], data[3]]);
        Some(Header {
            ver,
            msg_type,
            tkl,
            code,
            msg_id,
        })
    }
}

impl CoAPFrame {

    pub fn new(header: Header, options: BTreeMap<u16, Vec<Vec<u8>>>, payload: Vec<u8>) -> Self {
        let tkl = header.tkl;
        CoAPFrame {
            header,
            token: generate_coap_token(tkl as usize),
            options,
            ff: 0xFF,
            payload,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let header = Header::from_bytes(&bytes[..4]).unwrap();
        let token = &bytes[4..(4 + header.tkl as usize)];
        let mut options: BTreeMap<u16, Vec<Vec<u8>>> = BTreeMap::new();
        let mut offset = 4 + header.tkl as usize;
        let mut payload: Vec<u8> = Vec::new();
        if bytes.len() > offset {
            if bytes[offset] == 0xFF {
                payload = bytes[offset + 1..].to_vec();
            } else {
                let mut number: u16 = 0;
                loop {
                    if bytes[offset] == 0xFF {
                        break;
                    }
                    let first_byte = bytes[offset];
                    let delta = (first_byte >> 4) & 0xF;
                    let vl = first_byte & 0xF;

                    let mut length: u16 = vl as u16;

                    if delta < 13 {
                        number += delta as u16;
                        if vl < 13 {
                            offset += 1;
                        } else if vl == 13 {
                            length = 13 + bytes[offset + 1] as u16;
                            offset += 2;
                        } else {
                            length = 269
                                + u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                            offset += 3;
                        }
                    } else if delta == 13 {
                        number = number + 13 + bytes[offset + 1] as u16;
                        if vl < 13 {
                            offset += 2;
                        } else if vl == 13 {
                            length = 13 + bytes[offset + 2] as u16;
                            offset += 3;
                        } else {
                            length = 269
                                + u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
                            offset += 4;
                        }
                    } else {
                        number = number
                            + 269
                            + u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);

                        if vl < 13 {
                            offset += 3;
                        } else if vl == 13 {
                            length = 13 + bytes[offset + 3] as u16;
                            offset += 4;
                        } else {
                            length = 269
                                + u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                            offset += 5;
                        }
                    };
                    let value = bytes[offset..(offset + length as usize)].to_vec();
                    offset += length as usize;

                    let option_list = options.get(&number);
                    match option_list {
                        None => {
                            options.insert(number, vec![value]);
                        }
                        Some(option_vec) => {
                            let mut new_op = option_vec.clone();
                            new_op.push(value);
                            options.insert(number, new_op);
                        }
                    }
                }
                if bytes.len() > offset && bytes[offset] == 0xFF {
                    payload = bytes[offset + 1..].to_vec();
                }
            }
        }
        CoAPFrame {
            header,
            token: token.to_vec(),
            options,
            ff: 0xFF,
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.header.to_bytes()[..]);

        //push token
        buf.extend_from_slice(&self.token);
        let mut option_vec: Vec<CoapOption> = Vec::new();
        let mut delta;
        let mut before = 0;
        for ele in &self.options {
            let values = ele.1;
            for v in values {
                delta = ele.0 - before;
                before = *ele.0;
                option_vec.push(CoapOption::new(delta, v.to_vec()))
            }
        }
        for ele in option_vec {
            buf.extend_from_slice(&ele.encode())
        }
        if !self.payload.is_empty() {
            buf.push(0xFFu8);
            buf.extend_from_slice(&self.payload);
        }
        buf
    }

    pub fn get_body(&self) -> Vec<u8> {
        self.payload.clone()
    }

    pub fn get_options(&self) -> BTreeMap<u16, Vec<Vec<u8>>> {
        self.options.clone()
    }
}

fn generate_coap_message_id() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn generate_coap_token(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let token: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    token
}

enum ContentFormat {
    TextPlain,
    ApplicationLinkFormat,
    ApplicationXml,
    ApplicationOctetStream,
    ApplicationExi,
    ApplicationJson,
}

impl TryFrom<u16> for ContentFormat {
    type Error = InvalidContentFormat;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ContentFormat::TextPlain),
            40 => Ok(ContentFormat::ApplicationLinkFormat),
            41 => Ok(ContentFormat::ApplicationXml),
            42 => Ok(ContentFormat::ApplicationOctetStream),
            47 => Ok(ContentFormat::ApplicationExi),
            50 => Ok(ContentFormat::ApplicationJson),
            _ => Err(InvalidContentFormat),
        }
    }
}

impl From<ContentFormat> for u16 {
    fn from(value: ContentFormat) -> u16 {
        match value {
            ContentFormat::TextPlain => 1,
            ContentFormat::ApplicationLinkFormat => 40,
            ContentFormat::ApplicationXml => 41,
            ContentFormat::ApplicationOctetStream => 42,
            ContentFormat::ApplicationExi => 47,
            ContentFormat::ApplicationJson => 50,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::{
        frame::{
            generate_coap_message_id, generate_coap_token, CoAPFrame, ContentFormat, Header,
            OptionEnum, MessageType
        },
        request::RequestMethod,
    };

    #[test]
    fn header_to_bytes() {
        let msg_id = generate_coap_message_id();
        let mut header = Header {
            ver: 1,
            msg_type: MessageType::Con as u8,
            tkl: 8,
            code: RequestMethod::Get as u8,
            msg_id: msg_id,
        };

        let mut bytes = header.to_bytes();
        assert_eq!(bytes[0], 0b0100_1000);
        assert_eq!(bytes[1], RequestMethod::Get as u8);
        assert_eq!(bytes[2..], msg_id.to_be_bytes());

        header.code = RequestMethod::Post as u8;
        bytes = header.to_bytes();
        assert_eq!(bytes[1], RequestMethod::Post as u8);
        let hop = Header::from_bytes(&bytes);
        assert_eq!(hop.is_none(), false);
        let h = hop.unwrap();
        assert_eq!(h.code, RequestMethod::Post as u8);
        assert_eq!(h, header)
    }

    #[test]
    fn frame_to_bytes() {
        let token = generate_coap_token(8);
        let header = Header {
            ver: 1,
            msg_type: MessageType::Con as u8,
            tkl: token.len() as u8,
            code: RequestMethod::Get as u8,
            msg_id: generate_coap_message_id(),
        };

        let mut options = BTreeMap::new();
        options.insert(
            u16::from(OptionEnum::ContentFormat),
            vec![u16::from(ContentFormat::ApplicationJson)
                .to_be_bytes()
                .to_vec()],
        );
        options.insert(
            u16::from(OptionEnum::UriPath),
            vec![Vec::from("hello"), Vec::from("world")],
        );
        let packet = CoAPFrame {
            header: header,
            token: token.clone().into(),
            options: options,
            ff: 0xFF,
            payload: "{\"hello\":\"world\"}".into(),
        };

        let encode_buffer = packet.to_bytes();

        let frame = CoAPFrame::from_bytes(encode_buffer.to_vec());
        assert_eq!(frame, packet);
    }
}
