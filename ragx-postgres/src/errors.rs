use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("conexão string inválida: {0}")]
    InvalidConnectionString(String),

    #[error("esquema de conexão não suportada: {0}")]
    UnsupportedScheme(String),

    #[error("campo obrigatório ausente: {0}")]
    MissingField(String),

    #[error("erro interno: {0}")]
    InternalError(String),
}