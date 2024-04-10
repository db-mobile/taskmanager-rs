use elasticsearch::{BulkParts, Elasticsearch, http::transport::Transport};
use elasticsearch::http::request::JsonBody;
use futures::executor;
use serde_json::{json, Value};

pub fn write_bulk(data: &Vec<(String, String)>) -> Result <bool, elasticsearch::Error> {
    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);

    body.push(json!({"index": {"_id": "1"}}).into());
    body.push(json!({
        "id": 1,
        "user": "kimchy",
        "post_date": "2009-11-15T00:00:00Z",
        "message": "Trying out Elasticsearch, so far so good?"
    }).into());

    executor::block_on(index(body))
}

pub fn delete_bulk(data: &Vec<String>) -> Result <(), redis::RedisError> {
    let transport = Transport::single_node("https://localhost").expect("Error creating transport");
    let client = Elasticsearch::new(transport);

    Ok(())
}

async fn index(data: Vec<JsonBody<Value>>) -> Result<bool, elasticsearch::Error> {
    let transport = Transport::single_node("https://localhost")?;
    let client = Elasticsearch::new(transport);

    let response = client
        .bulk(BulkParts::Index("spryker_b2b_marketplace_dev_de_page"))
        .body(data)
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;

    Ok(response_body["errors"].as_bool().unwrap() == false)
}