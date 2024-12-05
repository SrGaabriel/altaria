use tokio::io::AsyncBufReadExt;
use tokio::net::TcpStream;

pub async fn read_line(stream: TcpStream) -> Result<String, std::io::Error> {
    let mut reader = tokio::io::BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(line)
}