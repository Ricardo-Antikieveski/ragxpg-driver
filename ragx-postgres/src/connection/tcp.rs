use tokio::net::TcpStream;
use crate::client::ConnectionConfig;
use crate::errors::ClientError;

pub async fn connect_tcp(config: &ConnectionConfig) -> Result<TcpStream, ClientError> {
    let host = config.host.as_ref().unwrap();
    let port = config.port.unwrap_or(5432);

    let addr = format!("{}:{port}", host);

    TcpStream::connect(addr)
        .await
        .map_err(|e| ClientError::InternalError(format!("TCP connection error: {}", e)))
}
