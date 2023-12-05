use std::collections::BTreeMap;

use crate::{common::{code_from, self}, error::InvalidResponseCode, frame::{MessageType, CoAPFrame, OptionEnum}};

/// response code
#[derive(Debug)]
enum ResponseCode {
    ///2.01
    Created,
    ///2.02
    Deleted,
    Valid,
    Changed,
    Content,

    //4.xx
    BadRequest,
    Unauthorized,
    BadOption,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    PreconditionFailed,
    RequestEntityTooLarge,
    UnsupportedContentFormat,

    //5.xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    ProxyingNotSupported
}

impl From<&ResponseCode> for u8 {
    fn from(value: &ResponseCode) -> Self {
        match value {
            ResponseCode::Created => code_from(2, 01),
            ResponseCode::Deleted => code_from(2, 02),
            ResponseCode::Valid => code_from(2, 03),
            ResponseCode::Changed => code_from(2, 04),
            ResponseCode::Content => code_from(2, 05),

            ResponseCode::BadRequest => code_from(4, 00),
            ResponseCode::Unauthorized => code_from(4, 01),
            ResponseCode::BadOption => code_from(4, 02),
            ResponseCode::Forbidden => code_from(4, 03),
            ResponseCode::NotFound => code_from(4, 04),
            ResponseCode::MethodNotAllowed => code_from(4, 05),
            ResponseCode::NotAcceptable => code_from(4, 06),
            ResponseCode::PreconditionFailed => code_from(4, 12),
            ResponseCode::RequestEntityTooLarge => code_from(4, 13),
            ResponseCode::UnsupportedContentFormat => code_from(4, 15),

            ResponseCode::InternalServerError => code_from(5, 00),
            ResponseCode::NotImplemented => code_from(5, 01),
            ResponseCode::BadGateway => code_from(5, 02),
            ResponseCode::ServiceUnavailable => code_from(5, 03),
            ResponseCode::GatewayTimeout => code_from(5, 04),
            ResponseCode::ProxyingNotSupported => code_from(5, 05)
        }
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = InvalidResponseCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x41 => Ok(ResponseCode::Created),
            0x42 => Ok(ResponseCode::Deleted),
            0x43 => Ok(ResponseCode::Valid),
            0x44 => Ok(ResponseCode::Changed),
            0x45 => Ok(ResponseCode::Content),

            0x80 => Ok(ResponseCode::BadRequest),
            0x81 => Ok(ResponseCode::Unauthorized),
            0x82 => Ok(ResponseCode::BadOption),
            0x83 => Ok(ResponseCode::Forbidden),
            0x84 => Ok(ResponseCode::NotFound),
            0x85 => Ok(ResponseCode::MethodNotAllowed),
            0x86 => Ok(ResponseCode::NotAcceptable),
            0x8C => Ok(ResponseCode::PreconditionFailed),
            0x8D => Ok(ResponseCode::UnsupportedContentFormat),
            0x8F => Ok(ResponseCode::RequestEntityTooLarge),

            0xA0 => Ok(ResponseCode::InternalServerError),
            0xA1 => Ok(ResponseCode::NotImplemented),
            0xA2 => Ok(ResponseCode::BadGateway),
            0xA3 => Ok(ResponseCode::ServiceUnavailable),
            0xA4 => Ok(ResponseCode::GatewayTimeout),
            0xA5 => Ok(ResponseCode::ProxyingNotSupported),

            _ => Err(InvalidResponseCode)
        }
    }
}

#[derive(Debug)]
pub struct Response {
    message_type: MessageType,
    code: ResponseCode,
    options: BTreeMap<u16, Vec<Vec<u8>>>,
    body: Vec<u8>
}

impl Response {
    
    pub fn get_type(&self) -> u8 {
        self.message_type.into()
    }

    pub fn get_code(&self) -> u8 {
        u8::from(&self.code)
    }

    pub fn get_code_str(&self) -> String {
        common::to_code_str(u8::from(&self.code))
    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn get_options(&self) -> BTreeMap<OptionEnum, &Vec<Vec<u8>>> {
        let mut options = BTreeMap::new();
        for ele in &self.options {
            let number = OptionEnum::try_from(*ele.0).expect("parse option number error");
            options.insert(number, ele.1);
        }
        options
    }

    pub fn from(buf: Vec<u8>) -> Response {
        let frame = CoAPFrame::from_bytes(buf);
        Response {
            message_type: frame.header.get_type().try_into().unwrap(),
            code: frame.header.get_code().try_into().unwrap(),
            options: frame.get_options(),
            body: frame.get_body()
        }
    }
}