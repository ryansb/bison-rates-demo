use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use ulid::Ulid;

use sdk_server_bison_rates::{
    error::{CreateBisonError, InternalFailureException, ListBisonError, ValidationException},
    input::{CreateBisonInput, ListBisonInput},
    model::BisonItem,
    output::{CreateBisonOutput, ListBisonOutput},
};

use crate::meta::version;

pub async fn create_bison(data: CreateBisonInput) -> Result<CreateBisonOutput, CreateBisonError> {
    tracing::info!("POST /bison body: {:?}", data);

    if data.name().is_empty() {
        return Err(CreateBisonError::ValidationException(ValidationException {
            message: "Field `name` must have contents".to_string(),
            field_list: None,
        }));
    }

    if data.herd().is_empty() {
        return Err(CreateBisonError::ValidationException(ValidationException {
            message: "Field `herd` must have contents".to_string(),
            field_list: None,
        }));
    }

    let id = Ulid::new().to_string();

    let rank = if 1 <= data.rank() && data.rank() < 100 {
        data.rank()
    } else {
        (rand::random::<i32>().abs() % 99) + 1
    };

    let ddb = Client::new(&aws_config::load_from_env().await);

    let req = ddb
        .put_item()
        .table_name(std::env::var("TABLE").unwrap())
        .item("pk", AttributeValue::S(format!("HERD#{}", data.herd())))
        .item("sk", AttributeValue::S(format!("ID#{}", id.clone())))
        .item("gsi1pk", AttributeValue::S(format!("HERD#{}", data.herd())))
        .item("gsi1sk", AttributeValue::S(format!("RANK#{:06}", rank)))
        .item("id", AttributeValue::S(id.clone()))
        .item("name", AttributeValue::S(data.name().into()))
        .item("herd", AttributeValue::S(data.herd().into()))
        .item("rank", AttributeValue::N(rank.to_string()));

    if let Err(e) = req.send().await {
        tracing::error!("Failed to write Bison: {:?}", e);
        return Err(CreateBisonError::InternalFailureException(
            InternalFailureException {
                code: Some("DynamoError".to_string()),
                message: format!("Failed to write Bison: {:?}", e),
            },
        ));
    }

    Ok(CreateBisonOutput::builder()
        .id(id)
        .version(version())
        .name(data.name())
        .herd(data.herd())
        .rank(rank)
        .build())
}

pub async fn list_bison(data: ListBisonInput) -> Result<ListBisonOutput, ListBisonError> {
    tracing::info!("GET /bison/{}", data.herd);

    let ddb = Client::new(&aws_config::load_from_env().await);

    let req = ddb
        .query()
        .table_name(std::env::var("TABLE").unwrap())
        .index_name("gsi1")
        .scan_index_forward(false)
        .key_condition_expression("gsi1pk = :h")
        .expression_attribute_values(":h", AttributeValue::S(format!("HERD#{}", data.herd())));

    match req.send().await {
        Err(e) => {
            tracing::error!("Failed to search for a the herd: {:?}", e);
            Err(ListBisonError::InternalFailureException(
                InternalFailureException {
                    code: Some("DynamoError".to_string()),
                    message: format!("Query failed: {:?}", e),
                },
            ))
        }
        Ok(result) => {
            let mut resp = ListBisonOutput::builder().version(version());
            if let Some(items) = result.items() {
                let mut max = 0;
                for i in items.iter() {
                    tracing::debug!("Received item: {:?}", i);

                    let r = i
                        .get("rank")
                        .unwrap()
                        .as_n()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

                    resp = resp.set_max(if r > max {
                        max = r;
                        Some(r)
                    } else {
                        Some(max)
                    });

                    let b = BisonItem {
                        id: i.get("id").unwrap().as_s().unwrap().to_string(),
                        name: i.get("name").unwrap().as_s().unwrap().to_string(),
                        tags: None,
                        rank: r,
                    };
                    tracing::info!("Bison: {:?}", &b);
                    resp = resp.members(b);
                }
            };
            Ok(resp.build())
        }
    }
}
