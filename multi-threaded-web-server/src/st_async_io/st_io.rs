use crate::async_handle_connection;
use tokio::net::TcpStream;

pub struct STAsyncIO;

impl STAsyncIO {
    pub async fn execute(&self, stream: TcpStream) {
        tokio::spawn(async move {
            async_handle_connection(stream).await;
        });
    }
}
