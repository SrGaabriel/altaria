use thiserror::Error;

pub mod beta;
pub mod format;
pub mod alpha;

#[derive(Debug, Error)]
pub enum HttpEncoderError {
}