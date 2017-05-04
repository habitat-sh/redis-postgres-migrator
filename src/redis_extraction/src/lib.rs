extern crate redis;
use redis::Commands;

pub fn extract_accounts() {
   let client = redis_client();
}

fn redis_client() -> redis::RedisResult<()>{
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
println!("{:?}", client);
    let con = try!(client.get_connection());

    Ok(())
}
