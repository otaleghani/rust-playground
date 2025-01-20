//use tokio::task;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async move { "sus".to_string() });
    let result = handle.await.unwrap();
    println!("{:?}", result);
}
