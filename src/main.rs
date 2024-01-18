// an example to setup a tls server.
// how to test:
// curl https://localhost:12345 -k
// or
// wget https://127.0.0.1:12345 --no-check-certificate -O - -q

// You should see "Hello, World!" on your console.

// to generate cert.pem and key.pem:
// openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -nodes -subj '/CN=localhost'

use bytes::Bytes;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::{rt::TokioExecutor, rt::TokioIo, server::conn::auto};
use native_tls::Identity;
use std::convert::Infallible;
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the server's socket
    let addr = "127.0.0.1:12345".to_string();
    let listen: TcpListener = TcpListener::bind(&addr).await?;

    let pem = include_bytes!("cert.pem");
    let key = include_bytes!("key.pem");
    let cert = Identity::from_pkcs8(pem, key)?;

    let tls_acceptor = native_tls::TlsAcceptor::builder(cert).build()?;
    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(tls_acceptor);

    loop {
        let (socket, _) = listen.accept().await?;
        let tls_acceptor = tls_acceptor.clone();
        let server = auto::Builder::new(TokioExecutor::new()); //http1 or http2

        tokio::spawn(async move {
            let tls_stream = tls_acceptor.accept(socket).await.unwrap();
            let io = TokioIo::new(tls_stream);
            let service = service_fn(hello);
            server.serve_connection(io, service).await.unwrap()
        });
    }
}
