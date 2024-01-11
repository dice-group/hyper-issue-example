use hyper::{body::HttpBody, Uri};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let endpoint = std::env::args().nth(1).unwrap();
    let endpoint: Uri = endpoint.parse().unwrap();

    let client = hyper::client::Client::new();

    let start = Instant::now();

    for _ in 0..200 {
        let mut resp = client.get(endpoint.clone()).await.unwrap();

        while let Some(Ok(chunk)) = resp.data().await {
            std::hint::black_box(chunk);
        }
    }

    let end = Instant::now();
    println!("took: {}s", end.duration_since(start).as_secs_f64());
}
