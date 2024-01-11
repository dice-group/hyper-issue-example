use bytes::Bytes;
use common::TokioIo;
use http_body_util::{BodyExt, Empty};
use hyper::{Request, Uri};
use tokio::{net::TcpStream, time::Instant};

#[tokio::main]
async fn main() {
    // mostly copied from hyper examples

    // read url from arguments
    let endpoint = std::env::args().nth(1).unwrap();
    let endpoint: Uri = endpoint.parse().unwrap();

    let start = Instant::now();

    for _ in 0..200 {
        // reusing the connection does not seem to work, not sure why
        // sometimes panics because ECANCELED

        let stream = TcpStream::connect(format!(
            "{}:{}",
            endpoint.host().unwrap(),
            endpoint.port_u16().unwrap_or(80)
        ))
        .await
        .unwrap();

        let io = TokioIo::new(stream);
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let req = Request::builder()
            .uri(endpoint.clone())
            .header(hyper::header::HOST, endpoint.authority().unwrap().as_str())
            .body(Empty::<Bytes>::new())
            .unwrap();

        let mut resp = sender.send_request(req.clone()).await.unwrap();

        while let Some(Ok(chunk)) = resp.frame().await {
            std::hint::black_box(chunk.data_ref());
        }
    }

    let end = Instant::now();
    println!("took: {}s", end.duration_since(start).as_secs_f64());
}
