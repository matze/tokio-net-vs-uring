use std::io;
use common_bench::{ADDRESS, RESPONSE};

fn main() -> io::Result<()> {
    tokio_uring::start(async {
        let listener = tokio_uring::net::TcpListener::bind(ADDRESS.parse().unwrap())?;

        loop {
            let (stream, _) = listener.accept().await?;

            tokio_uring::spawn(async move {
                let (result, _) = stream.write(RESPONSE).await;

                if let Err(err) = result {
                    eprintln!("Client connection failed: {}", err);
                }
            });
        }
    })
}
