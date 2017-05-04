extern crate redis;
use redis::Commands;

pub fn extract_accounts() -> redis::RedisResult<()> {
    let client = redis_client();
    let con = redis::Client::get_connection(&client).unwrap();
    let _ : () = try!(redis::cmd("KEYS").arg("*").query(&con));
//let _ : () = try!(con.set("my_key", 42));
//con.lpush("users", "Sylvanas")?;
//con.lpush("users", "Arthas")?;
    Ok(())
}

fn redis_client() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}
