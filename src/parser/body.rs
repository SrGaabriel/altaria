use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use crate::request::{HttpHeader, HttpHeaderMap};

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