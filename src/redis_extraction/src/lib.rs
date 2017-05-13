extern crate redis;
use redis::Commands;

pub fn extract_accounts(redis_addr: &str) -> Vec<String> {
    let account_keys = redis_keys(redis_addr, "account").expect("error extracting account keys");
    let account_data = redis::from_redis_value::<Vec<String>>(&account_keys).unwrap();
    account_data
}

pub fn redis_keys(redis_addr: &str, entity: &str) -> redis::RedisResult<(redis::Value)> {
    let client = redis_client(redis_addr);
    let con = redis::Client::get_connection(&client).unwrap();

    con.keys(format!("*{}:*", entity))
}

pub fn extract_origins(redis_addr: &str) -> Vec<String> {
    let origin_keys = redis_keys(redis_addr, "origin").expect("error extracting origin keys");
    redis::from_redis_value::<Vec<String>>(&origin_keys).unwrap()
}

fn redis_client(redis_addr: &str) -> redis::Client {
    let client = redis::Client::open(redis_addr);
    client.unwrap()
}
