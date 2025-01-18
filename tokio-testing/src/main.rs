use tokio::{io::AsyncReadExt, time};

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper() {
    println!("Sleeping");
    time::sleep(time::Duration::from_secs(2)).await;
    println!("Awaken");
}

async fn reader() {
    println!("Reading some data...");
    let mut f = tokio::fs::File::open("data.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    println!("Reading data, it's {} bytes long", contents.len());

    tokio::task::spawn_blocking(move || {
        println!("Computing fib(40)");
        fib(40);
        println!("Done computing");
    })
    .await
    .unwrap();
}

async fn run() {
    tokio::join!(sleeper(), reader(), reader(), reader(), reader(), reader(),);
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run();
    rt.block_on(future)
}
