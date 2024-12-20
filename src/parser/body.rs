use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

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