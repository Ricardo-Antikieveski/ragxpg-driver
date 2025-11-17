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

/// Representa que tipo de conexão será usada.
#[derive(Debug, Clone)]
pub enum ConnectionType{
    Tcp,
    Unix
}
