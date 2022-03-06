use std::io;

use tokio::io::AsyncWriteExt;

use common_bench::{ADDRESS, RESPONSE};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = stream.write_all(RESPONSE).await {
                eprintln!("Client connection failed: {}", err);
            }
        });
    }
}
