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
