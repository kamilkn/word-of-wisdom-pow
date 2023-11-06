use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::spawn;
use std::error::Error;
use log::{error, info};
use crate::server::tokio_server::book::BookService;

pub struct TokioServer {
    address: String,
    book_service: Arc<BookService>,
}

impl TokioServer {
    pub fn new(address: String, book_service: Arc<BookService>) -> TokioServer {
      TokioServer { address, book_service }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
      let listener = TcpListener::bind(&self.address).await?;
      info!("Server listening on {}", self.address);

      loop {
          let (stream, _) = listener.accept().await?;
          let book_service = self.book_service.clone();
          spawn(async move {
              match book_service.handle_request(stream).await {
                  Ok(_) => info!("Finish request"),
                  Err(e) => error!("Error: {}", e),
              }
          });
      }
  }
}
