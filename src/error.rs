use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidContentFormat;

impl Display for InvalidContentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CoAP error: invalid content format")
    }
}

impl Error for InvalidContentFormat {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}


#[derive(Debug)]
pub struct InvalidType;

impl Display for InvalidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CoAP error: invalid message type")
    }
}

impl Error for InvalidType {
    
}

#[derive(Debug)]
pub struct InvalidResponseCode;

impl Display for InvalidResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CoAP error: invalid response code")
    }
}

impl Error for InvalidResponseCode {
    
}