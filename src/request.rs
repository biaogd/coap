use std::{collections::BTreeMap, vec, net::UdpSocket};

use url::Url;

use crate::{frame::{
    Header, MessageType, CoAPFrame,
    OptionEnum
}, common::u16_to_bytes};

#[derive(Debug, Clone, Copy)]
pub enum RequestMethod {
    GET = 1,
    POST,
    PUT,
    DELETE,
}

pub struct CoapClient {
    uri: String,
    data_url: Url,
    timeout: u64,
    message_type: MessageType,
}

impl CoapClient {
    pub fn new(uri: String) -> Self {
        let data_url = Url::parse(&uri).expect("parse url error");
        if data_url.scheme() != "" && data_url.scheme() != "coap" && data_url.scheme() != "coaps" {
            panic!("url scheme not support, must coap or coaps")
        }
        CoapClient {
            uri,
            data_url,
            timeout: 247000,
            message_type: MessageType::Con,
        }
    }

    fn new_req(&self) -> Request {
        let host = self.data_url.host_str().unwrap();
        let port = match self.data_url.port() {
            Some(port) => port,
            None => 5683
        };
        let path = self.data_url.path();
        
        let mut options = BTreeMap::new();
        options.insert(u16::from(OptionEnum::UriHost), vec![Vec::from(host)]);
        options.insert(u16::from(OptionEnum::UriPort), vec![u16_to_bytes(port)]);
        if path.len() > 0 {
            let ps: Vec<Vec<u8>> = path.split("/")
            .filter(|f|f.len()>0)
            .map(|f| Vec::from(f)).collect();
            options.insert(u16::from(OptionEnum::UriPath,), ps);
        }
        if let Some(query) = self.data_url.query() {
            if query.len() > 0 {
                let qs = query.split("&").map(|f| {
                    Vec::from(f)
                }).collect();
                options.insert(u16::from(OptionEnum::UriQuery), qs);
            }
        }
        Request {
            message_type: MessageType::Con,
            code: RequestMethod::GET,
            scheme: self.data_url.scheme().to_owned(),
            host: host.to_owned(),
            port: port,
            options, 
            data_url: self.data_url.clone(),
            payload: vec![],
        }
    }

    pub fn get(&self) -> Vec<u8>{
        let req = self.new_req();
        req.send()
    }

    pub fn get_accept(&self, accept: u16) {

    }
}

struct Request {
    message_type: MessageType,
    code: RequestMethod,
    scheme: String,
    host: String,
    port: u16,
    options: BTreeMap<u16, Vec<Vec<u8>>>,
    data_url: Url,
    payload: Vec<u8>,
}

impl Request {

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.payload = body;
    }

    pub fn set_type(&mut self, message_type: MessageType) {
        self.message_type = message_type;
    }

    pub fn set_code(&mut self, code: RequestMethod) {
        self.code = code;
    }
    
    fn to_frame(&self) -> CoAPFrame {
        let header = Header::new(
            self.message_type.into(),
            self.code as u8
        );

        CoAPFrame::new(header, self.options.clone(), self.payload.clone())
    }

    fn send(&self) -> Vec<u8>{
        let frame = self.to_frame();
        let socket = UdpSocket::bind("0.0.0.0:0").expect("client bind error");
        socket.connect(format!("{}:{}", self.host, self.port)).expect("client connect error");
        let send_size = socket.send(&frame.to_bytes()).expect("send coap message error");
        println!("send size: {}", send_size);
        let mut buf = [0u8;1024];
        let recv_len = socket.recv(&mut buf).expect("udp recv error");

        let data = &buf[..recv_len];
        Vec::from(data)
    }
}