# RagXPG Driver

Este projeto é um driver PostgreSQL desenvolvido completamente do zero
em Rust.\
O objetivo é entender e implementar todas as camadas necessárias para
estabelecer comunicação com o servidor PostgreSQL, sem utilizar crates
de alto nível como `sqlx` ou `tokio-postgres`.

A ideia central é estudar em profundidade:

-   parsing de connection string\
-   comunicação de rede de baixo nível\
-   protocolo binário do PostgreSQL\
-   autenticação\
-   troca de mensagens\
-   buffers e tratamento de pacotes\
-   design de APIs internas em Rust

O projeto está sendo escrito de forma modular, para que cada parte do
driver seja compreendida e construída com clareza.

## O que já está implementado

### 1. Parser da Connection String

A biblioteca recebe uma connection string no formato PostgreSQL, por
exemplo:

    postgres://user:pass@localhost:5432/database

O parser extrai e valida:

-   usuário\
-   senha\
-   host\
-   porta\
-   nome do banco de dados\
-   tipo de conexão (TCP ou Unix)

Se o host estiver presente → conexão TCP\
Se não houver host → conexão Unix Socket

### 2. Suporte a TCP

A conexão TCP é feita usando `tokio::net::TcpStream`, de forma
totalmente assíncrona.

### 3. Suporte a Unix Socket

Quando não há host na URL, o driver assume que a intenção é usar Unix
Socket.

### 4. Arquitetura Separada e Modular

    ragxpg-driver/
       ├── client.rs
       ├── connection/
       │      ├── tcp.rs
       │      └── unix.rs
       ├── protocol/
       ├── errors.rs
       ├── lib.rs
       └── main.rs

### 5. API pública atual

``` rust
let client = Client::from_connection_str("postgres://user:pass@localhost/db")?;
let conn = client.connect().await?;
```

## Exemplo de uso

``` rust
use ragxpg_driver::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::from_connection_str(
        "postgres://postgres:123456@localhost:5432/meubanco"
    ).expect("Erro ao parsear");

    let conn = client.connect().await.expect("Erro ao conectar");

    println!("Conexão estabelecida!");
}
```

## Próximas etapas (Roadmap)

-   StartupMessage\
-   Autenticação\
-   ReadyForQuery\
-   Simple Query\
-   Row parsing\
-   Prepared Statements\
-   Pool de Conexões\
-   Publicação no crates.io

## Licença

MIT License

Copyright (c) 2025 Ricardo Antikieveski

Permission is hereby granted, free of charge, to any person obtaining a
copy ...
