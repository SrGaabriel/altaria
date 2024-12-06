use crate::response::HttpResponse;

pub struct AlphaHttpEncoder {
}

impl AlphaHttpEncoder {
    pub fn new() -> Self {
        AlphaHttpEncoder {}
    }

    fn insert_header(&self, key: &str, value: &str, encoded: &mut Vec<u8>) {
        encoded.extend_from_slice(key.as_bytes());
        encoded.extend_from_slice(b": ");
        encoded.extend_from_slice(value.as_bytes());
        encoded.extend_from_slice(b"\r\n");
    }

    pub fn encode(&self, response: HttpResponse) -> crate::Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(b"HTTP/1.1 ");
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