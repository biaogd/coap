use crate::frame::CoAPFrame;

pub enum MessageType {
    Con,
    Non,
    Ack,
    Rst,
}

pub enum RequestMethod {
    GET = 1,
    POST,
    PUT,
    DELETE,
}

trait Request {
    fn get(&self);

    fn post(&self, body: &[u8]);
}

struct CoapClient {
    uri: String,
    timeout: u64,
    message_type: MessageType,
}

impl CoapClient {
    pub fn new(uri: String) -> Self {
        CoapClient {
            uri,
            timeout: 247000,
            message_type: MessageType::Ack,
        }
    }

    fn to_packet(&self) -> CoAPFrame {
        todo!()
    }
}

impl Request for CoapClient {
    fn get(&self) {}

    fn post(&self, body: &[u8]) {
        todo!()
    }
}
