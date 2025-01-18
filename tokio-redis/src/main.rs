use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("helo", "world".into()).await?;
    let result = client.get("helo").await?;

    println!("got value from server; result={:?}", result);
    Ok(())
}
