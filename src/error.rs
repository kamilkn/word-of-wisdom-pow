use crate::client::client::ClientError;
use crate::server::std_server::book::BookError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Server: {0}")]
    Book(#[from] BookError),
    #[error("Client: {0}")]
    Client(#[from] ClientError),
}
