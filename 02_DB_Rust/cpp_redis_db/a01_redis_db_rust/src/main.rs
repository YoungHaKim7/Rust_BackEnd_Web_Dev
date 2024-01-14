use redis::{from_redis_value, streams::StreamRangeReply, Client, Commands, RedisError};

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

    // 4) xrevrange the read stream
    let result: Option<StreamRangeReply> = con.xrevrange("my_stream", "+", 10)?;
    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("->> xrevrange stream entity: {}", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!(" ->> {}: {}", name, from_redis_value::<String>(value)?);
            }
        }
        println!();
    }

    // 7) Final wait & cleanup
    con.del("my_key")?;
    con.del("my_stream")?;

    println!("->> the end");

    Ok(())
}
