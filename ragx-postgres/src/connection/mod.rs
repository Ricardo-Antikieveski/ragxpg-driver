pub mod tcp;
pub mod unix;

use ragx_core::connection_config::*;

use ragx_core::errors::ClientError;

use tcp::connect_tcp;
use unix::connect_unix;

pub enum ConnectionStream {
    Tcp(tokio::net::TcpStream),
    Unix(tokio::net::UnixStream),
}

pub struct Connection {
    pub stream: ConnectionStream,
    pub config: ConnectionConfig,
}

impl Connection {
    pub async fn connect(config: &ConnectionConfig) -> Result<Self, ClientError> {
        match config.connection {
            ConnectionType::Tcp => {
                let stream = connect_tcp(config).await?;
                Ok(Self {
                    stream: ConnectionStream::Tcp(stream),
                    config: config.clone(),
                })
            }
            ConnectionType::Unix => {
                let stream = connect_unix(config).await?;
                Ok(Self {
                    stream: ConnectionStream::Unix(stream),
                    config: config.clone(),
                })
            }
        }
    }
}
