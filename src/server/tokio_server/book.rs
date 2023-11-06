use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use log::info;
use crate::common::{pow::{PowError, PowProvider, PowSolution}, store::BookStore};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BookError {
    #[error("invalid message format")]
    InvalidClientMessageError,
    #[error("failed to check solution: {0}")]
    SolutionCheckError(#[from] PowError),
    #[error("no quotes in storage")]
    NoQuotesAvailable,
    #[error("failed to write quote: {0}")]
    StreamError(#[from] std::io::Error),
}

pub struct BookService {
    store: BookStore,
    pow: PowProvider,
}

impl BookService {
    pub fn new(store: BookStore, pow: PowProvider) -> BookService {
        BookService { store, pow }
    }

    pub async fn handle_request(&self, mut stream: TcpStream) -> Result<(), BookError> {
        let challenge = self.pow.generate_challenge()?;
        stream.write_all(challenge.as_slice()).await?;
        stream.flush().await?;
        info!("Challenge has been sent");

        let mut buffer = [0; 16];
        stream.read_exact(&mut buffer).await?;
        if buffer.len() != 16 {
            return Err(BookError::InvalidClientMessageError);
        }
        let challenge = buffer[8..].to_vec();
        let nonce = u64::from_le_bytes(buffer[..8].try_into().unwrap());
        let solution = PowSolution::new(challenge, nonce);
        self.pow.check_solution(&solution).map_err(BookError::SolutionCheckError)?;

        let quote = self.store.get_random_quote().ok_or(BookError::NoQuotesAvailable)?;
        stream.write_all(quote.as_bytes()).await.map_err(Into::into)
    }
}
