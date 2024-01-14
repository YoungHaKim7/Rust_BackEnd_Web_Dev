use std::{error::Error, time::Duration};
use tokio::time::sleep;

use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};

// region:    Notes
//
// Docker redis command: docker run --name redis_1 --rm -p 6379:6379 -it redis:6 -- --loglevel verbose
// Cargo  watch command: cargo watch -q -c -x 'run -q'
//         Crates redis: https://crates.io/crates/redis
//
// endregion: Notes

// Only uses one thread in runtime
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1) Create Connection
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_tokio_connection().await?;

    // 2) Set / Get Key
    con.set("First the key", "next the World!!").await?;
    let result: String = con.get("First the key").await?;
    println!("What is returned for key: \"First the Key\" - Result: \"{result}\"");

    // 3) xadd to redis stream
    con.xadd(
        "The first stream",
        "*",
        &[("name", "name-01"), ("title", "title-01")],
    )
    .await?;
    let len: i32 = con.xlen("The first stream").await?;
    println!("The length of stream \"The first stream\" is {len}");

    // 4) xrevrange the read stream
    let result: Option<StreamRangeReply> = con
        .xrevrange_count("The first stream", "+", "-", 10)
        .await?;
    if let Some(reply) = result {
        for stream_id in reply.ids {
            println!("--> xrevrange stream entity: {}", stream_id.id);
            for (name, value) in stream_id.map.iter() {
                print!(" --> {}:{}", name, from_redis_value::<String>(value)?);
            }
        }
    }

    // 5) Blocking xread (blocking from redis standpoint but not for the caller)
    tokio::spawn(async {
        let client = Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_tokio_connection().await.unwrap();

        loop {
            let opts = StreamReadOptions::default().count(1).block(0);
            let result: Option<StreamReadReply> = con
                .xread_options(&["The first stream"], &["$"], &opts)
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
    // Sleeping to wait eveything above is completed since we are everything in the same process
    sleep(Duration::from_millis(100)).await;
    con.xadd(
        "The first stream",
        "*",
        &[("name", "name-02"), ("title", "title-02")],
    )
    .await?;
    sleep(Duration::from_millis(100)).await;
    con.xadd(
        "The first stream",
        "*",
        &[("name", "name-03"), ("title", "title-03")],
    )
    .await?;

    // 7) Final wait and cleanup
    sleep(Duration::from_millis(10000)).await;
    con.del("First the key").await?;
    con.del("The first stream").await?;

    println!(" -->> the end");

    Ok(())
}
