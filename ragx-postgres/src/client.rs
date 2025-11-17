use url::Url;

use crate::connection::Connection;
use crate::errors::ClientError;

/// Representa que tipo de conexão será usada.
#[derive(Debug, Clone)]
pub enum ConnectionType{
    Tcp,
    Unix
}

/// Resultado do parse da connection String
#[derive(Debug, Clone)]
pub struct ConnectionConfig{
    pub username: String,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database: String,
    pub connection: ConnectionType,
}

/// Client principal do Driver
#[derive(Debug, Clone)]
pub struct Client{
    pub config: ConnectionConfig,
}


impl Client {
    pub async fn connect(&self) -> Result<Connection, ClientError> {
        Connection::connect(&self.config).await
    }

    /// Cria o client a partir de uma connection string estilo Postgres:
    /// postgres://user:pass@localhost:5432/mydb
    pub fn from_connection_str(conn_str: &str) -> Result<Self, ClientError> {
        let config = ConnectionConfig::from_connection_str(conn_str)?;
        Ok(Self { config })
    }
}

impl ConnectionConfig{
    pub fn from_connection_str(conn_str: &str) -> Result<Self, ClientError>{
        let url = Url::parse(conn_str)
            .map_err(|_| ClientError::InvalidConnectionString(conn_str.to_string()))?;

        //validar schema
        match url.scheme() {
            "postgres" | "postgresql" => {}
            other => {
                return Err(ClientError::UnsupportedScheme(other.to_string()));
            }
        }

        // username
        let username = if url.username().is_empty() {
            String::from("postgres")
        } else {
            url.username().to_string()
        };

        /*if username.is_empty(){
            return Err(ClientError::MissingField("username".to_string()));
        }*/

        // Password opcional
        let password = url.password().map(|s| s.to_string());

        // Host
        let host = url.host_str().map(|s| s.to_string());

        // Porta ( Se a conexão for TCP)
        let port = if let Some(p) = url.port() {
            Some(p)
        } else if host.is_some() {
            Some(5432) // default do Postgres
        } else {
            None // Unisocket não usa porta
        };

        // DataBase
        let path = url.path().trim_start_matches('/');
        if path.is_empty(){
            return Err(ClientError::MissingField("database".into()));
        }
        let database = path.to_string();

        // Detecta TCP ou UnixSocket
        let connection = if host.is_some(){
            ConnectionType::Tcp
        }else{
            ConnectionType::Unix
        };

        Ok(Self{
            username,
            password,
            host,
            port,
            database,
            connection,
        })
    }
}