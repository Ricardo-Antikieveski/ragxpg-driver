use tokio::net::UnixStream;
use ragx_core::connection_config::ConnectionConfig;
use ragx_core::errors::ClientError;

pub async fn connect_unix(config: &ConnectionConfig) -> Result<UnixStream, ClientError> {
    let port = config.port.unwrap_or(5432);

    let socket_path = format!("/var/run/postgresql/.s.PGSQL.{port}");

    UnixStream::connect(socket_path)
        .await
        .map_err(|e| ClientError::InternalError(format!("UnixSocket connection error: {}", e)))
}
