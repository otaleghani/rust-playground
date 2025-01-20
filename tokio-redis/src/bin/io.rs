use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let _ = read_first().await;
    let _ = read_all().await;
    let _ = write_to().await;
    let _ = write_all().await;
    let _ = copy_to().await;
    Ok(())
}

async fn read_first() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

async fn read_all() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;
    println!("The bytes: {:?}", buffer);
    Ok(())
}

async fn write_to() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;
    let n = file.write(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'", n);
    Ok(())
}

async fn write_all() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;
    let _ = file.write_all(b"some bytes").await?;
    Ok(())
}

async fn copy_to() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;
    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
