use std::io;

use futures_lite::AsyncWriteExt;
use glommio::net::TcpListener;

use common_bench::{ADDRESS, RESPONSE};

fn main() -> io::Result<()> {
    use glommio::LocalExecutorBuilder;
    LocalExecutorBuilder::default()
        .spawn(|| async move {
            let listener = TcpListener::bind(ADDRESS)?;

            loop {
                let mut stream = listener.accept().await?;

                glommio::spawn_local(async move {
                    stream.write_all(RESPONSE).await.unwrap();
                }).detach();
            }
        })
        .unwrap().join().unwrap()
}
