extern crate redis;
use redis::Commands;

pub fn extract_accounts() {
   let client = redis_client();
}

fn redis_client() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}
