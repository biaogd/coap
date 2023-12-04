use std::io::{self};

use request::CoapClient;

mod error;
mod frame;
mod request;
mod common;

fn main() -> io::Result<()> {

    let client = CoapClient::new(String::from("coap://coap.me/test"));
    let res = client.get();
    println!("{}", String::from_utf8(res).expect("invalid utf8 string"));
    Ok(())
}
