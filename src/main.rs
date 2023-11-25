use std::net::UdpSocket;
use std::io::{self};

use bytes::{Bytes, BufMut};
use rand::Rng;

/// coap version
const VER: u8 = 1;
const TOKEN_LEN: u8 = 8;

fn main() -> io::Result<()>{
    let get_h1 = Header {
        ver: VER,
        msg_type: MessageType::Con as u8,
        tkl: TOKEN_LEN,
        code: RequestMethod::GET as u8,
        msg_id: generate_coap_message_id()
    };
    
    let msg = CoAPFrame {
        header: get_h1,
        token: generate_coap_token(TOKEN_LEN.into()).into(),
        options: CoapOption::new(11, "5".into()).encode().into(),
        ff: 0xFF,
        payload: bytes::Bytes::new()
    };
    println!("{:?}", msg);

    let buf = msg.serialize();
    println!("{:?}", buf);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind local ip error");
    socket.connect("coap.me:5683")?;

    socket.send(&buf.to_vec()).expect("send message error");

    let mut buffer = [0; 1024];
    let bytes_read = socket.recv(&mut buffer)?;
    let res = &buffer[..bytes_read];
    println!("{:?}", res);
    Ok(())

}

#[derive(Debug)]
struct Header {
    ver: u8,
    msg_type: u8,
    tkl: u8,
    code: u8,
    msg_id: u16,
}

#[derive(Debug)]
struct CoAPFrame {
    header: Header,
    token: Bytes,
    options: Bytes,
    ff: u8,
    payload: Bytes,
}

/// CoAP
struct CoapOption {
    number: u16,
    value: Vec<u8>
}

impl CoapOption {
    
    fn new(number: u16, value: Vec<u8>) -> CoapOption {
        CoapOption { number, value }
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded_option: Vec<u8> = Vec::new();

        let delta = if self.number < 13 {
            self.number as u8
        }else if self.number < 269 {
            13
        }else {
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
            encoded_option.put_u16(self.number - 269 as u16)
        }

        if vl == 13 {
            encoded_option.push((length - 13) as u8);
        }

        if vl == 14 {
            encoded_option.put_u16((length - 269) as u16);
        }

        encoded_option.extend_from_slice(&self.value);

        encoded_option

    }

    fn decode(data: &[u8]) -> Option<CoapOption> {
        if data.is_empty() {
            return None;
        }

        let first_byte = data[0];
        let delta = (first_byte >> 4) & 0xF;

        let mut number = if delta < 13 {
            delta as u16
        } else if delta == 13 && data.len() > 1 {
            13 + data[1] as u16
        }else {

        }
        
        Some(())
    }
}

/// message type:
/// 0: Confirmable
/// 
enum MessageType {
    Con,
    Non,
    Ack,
    Rst,
}

enum RequestMethod {
    GET = 1,
    POST,
    PUT,
    DELETE
}

trait CoAPFrameConvertor {
    fn serialize(&self) -> Bytes;

    fn deserialize() -> Self;
}

impl CoAPFrameConvertor for CoAPFrame {

    fn deserialize() -> Self {
        todo!()
    }

    fn serialize(&self) -> Bytes {
        let mut buf = bytes::BytesMut::new();
        let header = &self.header;
        let t = header.ver << 6 | header.msg_type << 4 | header.tkl;
        buf.put_u8(t);
        buf.put_u8(header.code);
        buf.put_u16(header.msg_id);

        //push token
        buf.put(self.token.clone());
        buf.put(self.options.clone());
        if self.payload.len() > 0 {
            buf.put_u8(0xFFu8);
            buf.put(self.payload.clone());
        }
        buf.into()
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