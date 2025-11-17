use url::Url;
use ragx_core::connection_config::{ConnectionConfig, ConnectionType};
use ragx_core::errors::ClientError;
use crate::connection::Connection;

///
/// 1. DEFINIÇÃO DO CLIENT (driver Postgres)
///
#[derive(Debug, Clone)]
pub struct Client {
    pub config: ConnectionConfig,
}

impl Client {
    pub fn from_connection_str(conn_str: &str) -> Result<Self, ClientError> {
        let config = ConnectionConfig::from_connection_str(conn_str)?;
        Ok(Self { config })
    }

    pub async fn connect(&self) -> Result<Connection, ClientError> {
        Connection::connect(&self.config).await
    }
}

///
/// 2. TRAIT PARA CONFIG PARSER
///    (Importante: fica NO MESMO CRATE do Postgres!)
///
pub trait ConfigParser {
    fn from_connection_str(conn_str: &str) -> Result<ConnectionConfig, ClientError>;
}

///
/// 3. IMPLEMENTAÇÃO do parser Postgres para ConnectionConfig
///
impl ConfigParser for ConnectionConfig {
    fn from_connection_str(conn_str: &str) -> Result<ConnectionConfig, ClientError> {
        // 1. Parse da URL
        let url = Url::parse(conn_str)
            .map_err(|_| ClientError::InvalidConnectionString(conn_str.to_string()))?;

        // 2. Validar esquema
        match url.scheme() {
            "postgres" | "postgresql" => {}
            other => return Err(ClientError::UnsupportedScheme(other.to_string())),
        }

        // 3. Username (default: postgres)
        let username = if url.username().is_empty() {
            "postgres".to_string()
        } else {
            url.username().to_string()
        };

        // 4. Password
        let password = url.password().map(|p| p.to_string());

        // 5. Host
        let host = url.host_str().map(|h| h.to_string());

        // 6. Porta
        let port = if let Some(p) = url.port() {
            Some(p)
        } else if host.is_some() {
            Some(5432)
        } else {
            None
        };

        // 7. Nome do banco
        let path = url.path().trim_start_matches('/');
        if path.is_empty() {
            return Err(ClientError::MissingField("database".to_string()));
        }
        let database = path.to_string();

        // 8. TCP ou Unix
        let connection = if host.is_some() {
            ConnectionType::Tcp
        } else {
            ConnectionType::Unix
        };

        Ok(ConnectionConfig {
            username,
            password,
            host,
            port,
            database,
            connection,
        })
    }
}
