use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use serde_json::json;
use tokio::{spawn, sync::Semaphore};

#[derive(Parser)]
struct Cli {
    /// Number of maximum concurrent connections
    #[arg(short, long = "max-conn", default_value = "100")]
    max_conn: usize,

    /// Number of request to be performed
    #[arg(short, long, default_value = "1000")]
    count: usize,

    #[arg(short, long = "base-url", default_value = "http://localhost")]
    base_url: String,

    #[arg(short, long, default_value = "3000")]
    port: String,

    /// Time to live passed in request path
    #[arg(short, long, default_value = "5000")]
    ttl: usize,
}

#[derive(Default)]
struct Statistics {
    success: AtomicUsize,
    error: AtomicUsize,
}

const HEADER_NAME: &'static str = "Content-Type";
const HEADER_VALUE: &'static str = "application/json";

async fn perform_request(
    client: Client,
    url: &String,
    body: String,
    stats: &Statistics,
) -> Result<()> {
    match client
        .post(url)
        .header(HEADER_NAME, HEADER_VALUE)
        .body(body)
        .send()
        .await
    {
        Ok(_) => stats.success.fetch_add(1, Ordering::Relaxed),
        Err(_) => stats.error.fetch_add(1, Ordering::Relaxed),
    };
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let semaphre = Arc::new(Semaphore::new(args.max_conn));
    let statistics = Arc::new(Statistics::default());
    let formatted_url = format!("{}:{}/push/{}", args.base_url, args.port, args.ttl);
    let req_client = Client::new();
    let json = json!({
        "say":"Hello world!",
        "square":{"width":3000000,"height":3000000},
        "some":{"very":{"deep":{"nested":{"prop":69}}}},
        "ok":true,
        "sadness_level":null
    })
    .to_string();

    for i in 0..args.count {
        if i % 10000 == 0 {
            println!("Performed {i} requests");
        }
        let permit = semaphre.clone().acquire_owned().await.unwrap();
        let client = req_client.clone();
        let url = formatted_url.clone();
        let body = json.clone();
        let stats = statistics.clone();

        spawn(async move {
            perform_request(client, &url, body, &stats).await.unwrap();
            drop(permit);
        });
    }

    spawn(async move {
        while semaphre.available_permits() != args.max_conn {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .unwrap();

    println!(
        "Sucesses: {}\nErrors: {}\n",
        statistics.success.load(Ordering::Relaxed),
        statistics.error.load(Ordering::Relaxed)
    );

    Ok(())
}
