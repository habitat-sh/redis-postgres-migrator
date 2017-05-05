extern crate redis;
use redis::Commands;

pub fn extract_accounts(redis_addr: &str) -> Vec<String> {
    let account_keys = account_keys(redis_addr).unwrap();
    let account_data = redis::from_redis_value::<Vec<String>>(&account_keys).unwrap();
    account_data
}

pub fn account_keys(redis_addr: &str) -> redis::RedisResult<(redis::Value)> {
    let client = redis_client(redis_addr);
    let con = redis::Client::get_connection(&client).unwrap();

    let account_keys = con.keys("*account:*");
    account_keys
}

fn redis_client(redis_addr: &str) -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}
