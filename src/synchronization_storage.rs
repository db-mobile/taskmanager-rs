extern crate redis;

use redis::Commands;

pub fn write_bulk(data: &Vec<(String, String)>) -> Result <(), redis::RedisError> {
    let client = redis::Client::open("redis://127.0.0.1:16379/1")?;
    let mut con = client.get_connection()?;

    con.mset(&data)?;

    Ok(())
}

pub fn delete_bulk(data: &Vec<String>) -> Result <(), redis::RedisError> {
    let client = redis::Client::open("redis://127.0.0.1:16379/1")?;
    let mut con = client.get_connection()?;

    con.del(&data)?;

    Ok(())
}