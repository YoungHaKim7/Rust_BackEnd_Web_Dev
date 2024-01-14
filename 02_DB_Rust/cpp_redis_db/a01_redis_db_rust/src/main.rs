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

    // 3) xadd to redis stream
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-01"), ("title", "title 01")],
    )?;
    let len: i32 = con.xlen("my_stream")?;
    println!("-->> my_stream len {} \n", len);

    // 7) Final wait & cleanup
    con.del("my_key")?;

    println!("->> the end");

    Ok(())
}
