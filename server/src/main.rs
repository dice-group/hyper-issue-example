use bytes::{Bytes, BytesMut};
use common::TokioIo;
use futures_util::TryStreamExt;
use http_body_util::{combinators::BoxBody, BodyExt, Full, StreamBody};
use hyper::{
    body::{Frame, Incoming},
    server::conn::http1::Builder,
    service::service_fn,
    Request, Response, Result, StatusCode,
};
use std::{convert::Infallible, net::SocketAddr};
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

/// simulates work by spinning in a loop
fn simulate_work() -> usize {
    let mut ix = 0;
    for _ in 0..100_000 {
        ix += 1;
        std::hint::black_box(ix);
    }

    ix
}

/// Generates some bytes
fn generate_data() -> Bytes {
    let mut buf = BytesMut::with_capacity(256);

    for _ in 0..1000 {
        std::hint::black_box(simulate_work());

        buf.extend_from_slice(&vec![b'a'; 10]); // *1 HERE
    }

    buf.freeze()
}

/// Regular route that sends a single chunk as `Full` body
async fn hyper_regular() -> Result<Response<BoxBody<Bytes, Infallible>>> {
    let (tx, rx) = oneshot::channel();

    std::thread::spawn(move || {
        let data = std::hint::black_box(generate_data());
        let _ = tx.send(Ok::<_, Infallible>(data));
    });

    let response = rx.await.unwrap().unwrap();
    Ok(Response::new(Full::new(response).map_err(|e| match e {}).boxed()))
}

/// Same as regular route, also sends a single chunk but this time as a `StreamBody`
async fn hyper_stream() -> Result<Response<BoxBody<Bytes, Infallible>>> {
    let (tx, rx) = mpsc::unbounded_channel();

    std::thread::spawn(move || {
        let data = std::hint::black_box(generate_data());
        let _ = tx.send(Ok::<_, Infallible>(data));
    });

    let resp_stream = UnboundedReceiverStream::new(rx);
    let stream_body = StreamBody::new(resp_stream.map_ok(Frame::data));
    let boxed_body = stream_body.boxed();

    Ok(Response::builder().status(StatusCode::OK).body(boxed_body).unwrap())
}

/// Forwards to:
///     - hyper_stream if on route /stream
///     - to hyper_regular if on route /regular
async fn hyper_route(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>> {
    if req.uri().path().ends_with("/stream") {
        hyper_stream().await
    } else if req.uri().path().ends_with("/regular") {
        hyper_regular().await
    } else {
        unreachable!()
    }
}

#[tokio::main]
async fn main() {
    // basically one-to-one copied from hyper examples

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{addr} routes /stream and /regular");
    loop {
        let (tcp, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(tcp);

        tokio::task::spawn(async move {
            if let Err(err) = Builder::new().serve_connection(io, service_fn(hyper_route)).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
