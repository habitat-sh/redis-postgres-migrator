extern crate redis;
use redis::Commands;

pub fn extract_accounts() -> Vec<String> {
    let account_keys = account_keys().unwrap();
    let account_data = redis::from_redis_value::<Vec<String>>(&account_keys).unwrap();
    account_data
}

pub fn account_keys() -> redis::RedisResult<(redis::Value)> {
    let client = redis_client();
    let con = redis::Client::get_connection(&client).unwrap();

    let account_keys = con.keys("*account:*");
    account_keys
}

fn redis_client() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}
