use clap::Parser;
use tokio::io::AsyncWriteExt;

#[derive(Parser)]
struct Args {
    /// Use tokio-uring
    #[clap(long)]
    uring: bool,
}

static RESPONSE: &'static str =
    "HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello world!";

fn listen_regular(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Using tokio, listening on: {addr}");

    let rt = tokio::runtime::Builder::new_current_thread().enable_io().build()?;

    rt.block_on(async {
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;

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
    let args = Args::parse();
    let addr = "0.0.0.0:8080";

    if args.uring {
        listen_uring(&addr)
    } else {
        listen_regular(&addr)
    }
}
