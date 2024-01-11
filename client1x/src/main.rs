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

    println!("Using path: {}", endpoint.path());

    let stream = TcpStream::connect(format!(
        "{}:{}",
        endpoint.host().unwrap(),
        endpoint.port_u16().unwrap_or(80)
    ))
    .await
    .unwrap();
    stream.set_nodelay(true).unwrap();
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    for i in 0..200 {
        let endpoint : Uri = format!("{}://{}{}",
            endpoint.scheme_str().unwrap(),
            endpoint.authority().unwrap().as_str(),
            format!("{}?{}", endpoint.path(), i).as_str()).parse().unwrap();
        let req = Request::builder()
            .uri(endpoint.clone())
            .header(hyper::header::HOST, endpoint.authority().unwrap().as_str())
            .body(Empty::<Bytes>::new())
            .unwrap();

        // Cannot just call send_request after prior response finished
        // MUST call poll_ready and wait for the connection to be marked as ready
        // This will typically be a no-op, but failing to call this leads to a race
        // conditionw where hyper will call this and will fail if the connection
        // state variable has not been updated to indicate the connection is ready
        // for a new request
        // Consider that reading the last byte off a stream does not have to happen
        // at the exact moment (or in the same thread) as the code that is updating
        // the connection state variable
        while futures::future::poll_fn(|ctx| sender.poll_ready(ctx))
                .await
                .is_err()
        {
            // This gets hit when the connection for HTTP/1.1 faults
            panic!("Connection ready check threw error - connection has disconnected, should reconnect");
        }

        let mut resp = match sender.send_request(req.clone()).await {
            Ok(resp) => resp,
            Err(err) => {
                println!("Failed to send request #{}: {:?}", i, err);
                return;
            }
        };

        while let Some(Ok(chunk)) = resp.frame().await {
            std::hint::black_box(chunk.data_ref());
        }
    }

    let end = Instant::now();
    println!("took: {}s", end.duration_since(start).as_secs_f64());
}
