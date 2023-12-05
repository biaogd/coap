use std::io::{self};

use request::CoapClient;

use crate::frame::MessageType;

mod error;
mod frame;
mod request;
mod response;
mod common;

fn main() -> io::Result<()> {

    let client = CoapClient::new(String::from("coap://coap.me/test"));
    let res = client.get();
    println!("{}", String::from_utf8(res.get_body().to_vec()).expect("invalid utf8 string"));
    println!("code={}, type={:?}", res.get_code_str(), MessageType::try_from(res.get_type()).unwrap());
    println!("options = {:?}", res.get_options());
    Ok(())
}
