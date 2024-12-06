pub mod beta;
pub mod format;
pub mod alpha;

#[derive(Debug)]
pub struct HttpEncoderError {
    pub message: String
}