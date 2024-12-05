use crate::response::HttpResponse;

pub mod beta;
pub mod format;
pub mod alpha;

pub trait HttpEncoder {
    fn encode(&self, response: HttpResponse) -> Result<Vec<u8>, HttpEncoderError>;
}

#[derive(Debug)]
pub struct HttpEncoderError {
    pub message: String
}