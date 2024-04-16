use elasticsearch::{BulkParts, Elasticsearch, http::transport::Transport};
use elasticsearch::http::request::JsonBody;
use futures::executor;
use serde_json::{json, Value};

pub fn write_bulk(data: &Vec<(String, Value)>) -> Result <bool, elasticsearch::Error> {
    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(data.len() * 2);

    for (key, value) in data {
        let index_body = json!({"index": {"_id": key}});

        body.push(index_body.into());
        body.push(value.clone().into());
    }

    let result = executor::block_on(index(body));

    match result {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

pub fn delete_bulk(data: &Vec<String>) -> Result <(), elasticsearch::Error> {
    let transport = Transport::single_node("http://localhost").expect("Error creating transport");
    let client = Elasticsearch::new(transport);

    Ok(())
}

async fn index(data: Vec<JsonBody<Value>>) -> Result<bool, elasticsearch::Error> {
    let transport = Transport::single_node("http://localhost:9200")?;
    let client = Elasticsearch::new(transport);

    let response = client
        .bulk(BulkParts::Index("spryker_b2b_marketplace_dev_de_page"))
        .body(data)
        .send()
        .await?;
println!("{:?}", response.status_code());
    if response.status_code().is_success() {
        Ok(true)
    } else {
        Ok(false)
    }
}