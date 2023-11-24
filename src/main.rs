use bytes::{Bytes, BufMut};

/// coap version
const VER: u8 = 1;

fn main() {
    let a = 1u8;
    let type_v = 1u8;
    println!("{:b}", a<<6 | type_v << 4);
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

/// message type:
/// 0: Confirmable
/// 
enum MessageType {
    Con,
    Non,
    Ack,
    Rst,
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
        buf.put_bytes(1, 2);
        buf.into()
    }
}