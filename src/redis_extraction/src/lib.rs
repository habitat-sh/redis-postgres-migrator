extern crate redis;
use redis::Commands;

pub fn extract_accounts() -> redis::RedisResult<(redis::Value)> {
    let client = redis_client();
    let con = redis::Client::get_connection(&client).unwrap();

    let keys = con.keys("*");
    println!("{:?}", keys);

    keys
}

fn redis_client() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}
