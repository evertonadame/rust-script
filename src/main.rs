use reqwest;
use serde::Deserialize;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(make_request()).unwrap();
    println!("Hello, world!");
}


async fn make_request() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

    println!("body = {:?}", body);
    Ok(())
}