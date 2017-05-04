extern crate redis;
use redis::Commands;

pub fn extract_accounts() -> redis::RedisResult<()> {
    let client = redis_client();
    let con = redis::Client::get_connection(&client).unwrap();
con.lpush("users", "Sylvanas")?;
con.lpush("users", "Arthas")?;
    Ok(())
}

fn redis_client() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}
