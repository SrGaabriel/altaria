use crate::encoder::{HttpEncoder, HttpEncoderError};
use crate::response::HttpResponse;

pub struct BetaHttpEncoder {

}

impl BetaHttpEncoder {
    fn insert_header(&self, key: &str, value: &str, encoded: &mut Vec<u8>) {
        encoded.extend_from_slice(key.as_bytes());
        encoded.extend_from_slice(b": ");
        encoded.extend_from_slice(value.as_bytes());
        encoded.extend_from_slice(b"\r\n");
    }
}

impl HttpEncoder for BetaHttpEncoder {
    fn encode(&self, response: HttpResponse) -> Result<Vec<u8>, HttpEncoderError> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(b"HTTP/2 ");
        encoded.extend_from_slice(response.status_code.code().to_string().as_bytes());
        encoded.extend_from_slice(b"\r\n");

        for (key, value) in response.headers.iter() {
            self.insert_header(&key.name(), value, &mut encoded);
        }

        encoded.extend_from_slice(b"\r\n");
        encoded.extend_from_slice(&response.body);

        Ok(encoded)
    }
}