use std::time::Duration;

use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client, Commands, RedisError,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    // 1) Create Connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_tokio_connection().await?;

    // 2) Set / Get Key
    con.set("my_key", b"Hello world!").await?;
    let result: String = con.get("my_key").await?;
    println!("--> my_key : {} \n", result);

    // 3) xadd to redis stream
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-01"), ("title", "title 01")],
    )
    .await?;
    let len: i32 = con.xlen("my_stream").await?;
    println!("-->> my_stream len {} \n", len);

    // 4) xrevrange the read stream
    let result: Option<StreamRangeReply> = con
        .xrevrange_count("The first stream", "+", "-", 10)
        .await?;
    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("->> xrevrange stream entity: {}", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                println!(" ->> {}: {}", name, from_redis_value::<String>(value)?);
            }
        }
        println!();
    }

    // 5) Blocking xread
    tokio::spawn(async {
        let client = Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_tokio_connection().await.unwrap();

        loop {
            let opts = StreamReadOptions::default().count(1).block(0);
            let result: Option<StreamReadReply> = con
                .xread_options(&["my_stream"], &["$"], &opts)
                .await
                .unwrap();
            if let Some(reply) = result {
                for stream_key in reply.keys {
                    println!("--> xread block: {}", stream_key.key);
                    for stream_id in stream_key.ids {
                        println!(" --> StreamID: {:?}", stream_id);
                    }
                }
                println!();
            }
        }
    });
    // 6) Add some stream entries
    sleep(Duration::from_millis(100)).await;
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-02"), ("title", "title 02")],
    )
    .await?;
    sleep(Duration::from_millis(100)).await;
    con.xadd(
        "my_stream",
        "*",
        &[("name", "name-03"), ("title", "title 03")],
    )
    .await?;

    // 7) Final wait & cleanup
    sleep(Duration::from_millis(1000)).await;
    con.del("my_key").await?;
    con.del("my_stream").await?;

    println!("->> the end");

    Ok(())
}
