use reqwest::{Client, Url};
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    // read url from arguments
    let endpoint = std::env::args().nth(1).unwrap();
    let endpoint: Url = endpoint.parse().unwrap();

    let client = Client::new();

    let start = Instant::now();

    for _ in 0..200 {
        // reqwest tells you to reuse the client
        // "The Client holds a connection pool internally, so it is advised that you create one and reuse it." - https://docs.rs/reqwest/0.11.23/reqwest/struct.Client.html

        let mut res = client.get(endpoint.clone()).send().await.unwrap();

        while let Ok(Some(chunk)) = res.chunk().await {
            std::hint::black_box(chunk);
        }
    }

    let end = Instant::now();
    println!("took: {}s", end.duration_since(start).as_secs_f64());
}
