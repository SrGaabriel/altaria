use crate::encoder::{HttpEncoder, HttpEncoderError};
use crate::response::HttpResponse;
use hpack::Encoder as HpackEncoder;

const END_HEADERS: u8 = 0x4;
const END_STREAM: u8 = 0x1;

pub struct BetaHttpEncoder<'a> {
    hpack_encoder: HpackEncoder<'a>,
}

impl<'a> BetaHttpEncoder<'a> {
    pub fn new() -> Self {
        BetaHttpEncoder {
            hpack_encoder: HpackEncoder::new()
        }
    }

    fn encode_headers(&self, response: &HttpResponse) -> Vec<u8> {
        let mut encoded_headers = Vec::new();

        for (key, value) in &response.headers {
            todo!();
            // self.hpack_encoder.encode_header_into(
            //     (key.name().as_bytes(), value.as_bytes()),
            //     &mut encoded_headers
            // ).expect("Failed to encode header");
        }

        encoded_headers
    }

    fn create_frame(&self, frame_type: u8, flags: u8, stream_id: u32, payload: &[u8]) -> Vec<u8> {
        let mut frame = Vec::with_capacity(9 + payload.len());

        let length = payload.len();
        frame.push(((length >> 16) & 0xFF) as u8);
        frame.push(((length >> 8) & 0xFF) as u8);
        frame.push((length & 0xFF) as u8);

        frame.push(frame_type);
        frame.push(flags);

        let stream_id_bytes = (stream_id & 0x7FFFFFFF).to_be_bytes();
        frame.extend_from_slice(&stream_id_bytes);

        frame.extend_from_slice(payload);

        frame
    }
}

impl<'a> HttpEncoder for BetaHttpEncoder<'a> {
    fn encode(&self, response: HttpResponse) -> Result<Vec<u8>, HttpEncoderError> {
        let mut encoded = Vec::new();

        let headers = response
            .headers
            .iter()
            .map(|(key, value)| (key.name(), value.to_string()))
            .collect::<Vec<_>>();

        let compressed_headers = self.encode_headers(&response);
        let headers_frame = self.create_frame(0x1, END_HEADERS, 1, &compressed_headers);
        encoded.extend_from_slice(&headers_frame);

        if !response.body.is_empty() {
            let data_frame = self.create_frame(0x0, END_STREAM, 1, &response.body);
            encoded.extend_from_slice(&data_frame);
        }

        Ok(encoded)
    }
}

impl Clone for BetaHttpEncoder<'_> {
    fn clone(&self) -> Self {
        BetaHttpEncoder {
            hpack_encoder: HpackEncoder::new()
        }
    }
}