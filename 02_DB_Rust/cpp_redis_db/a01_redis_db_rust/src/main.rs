use redis::{Client, Commands, RedisError};

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    // 1) Create Connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // 2) Set / Get Key
    con.set("my_key", b"Hello world!")?;

    println!("->> the end");
    Ok(())
}
