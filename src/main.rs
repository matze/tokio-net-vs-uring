use tokio::io::AsyncWriteExt;

static RESPONSE: &'static str =
    "HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello world!";

fn listen_regular(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Using tokio, listening on: {addr}");

    tokio::runtime::Runtime::new()?.block_on(async {
        let server = tokio::net::TcpListener::bind(&addr).await?;

        loop {
            let (mut stream, _) = server.accept().await?;

            tokio::spawn(async move {
                if let Err(err) = stream.write_all(RESPONSE.as_bytes()).await {
                    eprintln!("Client connection failed: {err}");
                }
            });
        }
    })
}

fn listen_uring(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Using tokio-uring, listening on: {addr}");

    tokio_uring::start(async {
        let listener = tokio_uring::net::TcpListener::bind(addr.parse()?)?;

        loop {
            let (stream, _) = listener.accept().await?;

            tokio_uring::spawn(async move {
                let (result, _) = stream.write(RESPONSE.as_bytes()).await;

                if let Err(err) = result {
                    eprintln!("Client connection failed: {err}");
                }
            });
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let use_uring = std::env::args().find(|a| a == "--uring").is_some();
    let addr = "0.0.0.0:8080";

    if use_uring {
        listen_uring(&addr)
    } else {
        listen_regular(&addr)
    }
}
