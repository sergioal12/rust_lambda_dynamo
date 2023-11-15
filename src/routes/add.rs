use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

/// how i can perform an dynamic insert in dynamo?
pub struct Item {
    pub ppal_key: String,
    pub data1: String,
    pub data2: Option<AttributeValue>,  
}

#[derive(Debug, partialEq)]
pub struct ItemOut {
    pub ppal_key: Option<AttributeValue>,
    pub data1: Option<AttributeValue>,
    pub data2: Option<AttributeValue>,
}


pub async fn add_item(client: &Client, item: Item, table: &String) -> Result<ItemOut, Error> {
    let ppal_key = AttributeValue::S(item.ppal_key);
    let data1 = AttributeValue::S(item.data1);
    let data2 = AttributeValue::S(item.data2);


    let request = client
        .put_item()
        .table_name(table)
        .item("ppal_key", ppal_key)
        .item("data1", data1)
        .item("data2", data2);

    println!("executing put item in dynamo");

    let resp = request.send().await?;

    let attributes = resp.attributes().unwrap();

    let ppal_key = attributes.get("ppal_key").cloned();
    let data1 = attributes.get("data1").cloned();
    let data2 = attributes.get("data2").cloned();

    println!("response, ppal_key {:?}, data1 {:?}, data2 {:?}", ppal_key, data1, data2);

    Ok(ItemOut{
        ppal_key,
        data1,
        data2
    })

}