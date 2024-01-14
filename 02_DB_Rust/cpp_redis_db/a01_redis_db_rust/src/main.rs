use redis::{Client, Commands, RedisError};

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    // 1) Create Connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    // 2) Set / Get Key
    con.set("my_key", b"Hello world!")?;
    let result: String = con.get("my_key")?;
    println!("--> my_key : {} \n", result);

    // 7) Final wait & cleanup
    con.del("my_key")?;

    println!("->> the end");

    Ok(())
}
