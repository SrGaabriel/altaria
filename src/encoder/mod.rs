use thiserror::Error;

pub mod format;
pub mod alpha;

#[derive(Debug, Error)]
pub enum HttpEncoderError {
}