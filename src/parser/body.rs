use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use crate::request::{HttpHeader, HttpHeaderMap};

pub struct LazyBodyReader {
    reader: BufReader<OwnedReadHalf>,
    buffer: Box<[u8]>,
    consumed: bool
}

impl LazyBodyReader {
    pub fn new(reader: BufReader<OwnedReadHalf>, content_length: usize) -> Self {
        LazyBodyReader {
            reader,
            buffer: vec![0; content_length].into_boxed_slice(),
            consumed: false
        }
    }

    pub async fn consume_all(mut self) -> Box<[u8]> {
        if self.consumed {
            return self.buffer;
        }
        self.reader.read_exact(&mut self.buffer).await.unwrap();
        self.consumed = true;
        self.buffer
    }

    pub async fn read_all(&mut self) -> &[u8] {
        if self.consumed {
            return &self.buffer;
        }
        self.reader.read_exact(&mut self.buffer).await.unwrap();
        self.consumed = true;
        &self.buffer
    }
}

pub(crate) async fn read_body_based_on_headers(headers: &HttpHeaderMap, reader: &mut BufReader<&mut TcpStream>) -> Vec<u8> {
    let content_length = headers.get(&HttpHeader::ContentLength);
    if let Some(content_length) = content_length {
        let content_length = content_length.parse::<usize>().unwrap();
        let mut limited_reader = reader.take(content_length as u64);
        let mut body = Vec::new();
        limited_reader.read_to_end(&mut body).await.unwrap();
        body
    } else {
        Vec::new()
    }
}