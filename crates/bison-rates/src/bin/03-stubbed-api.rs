use aws_smithy_http_server::Router;
use lambda_http::{run as run_in_lambda, Error};

use bison_rates::meta::{tower_tracer, version};
use sdk_server_bison_rates::{
    error::{CreateBisonError, ListBisonError},
    input::{CreateBisonInput, ListBisonInput},
    operation_registry::OperationRegistryBuilder,
    output::{CreateBisonOutput, ListBisonOutput},
};

pub async fn stubbed_create_bison(
    data: CreateBisonInput,
) -> Result<CreateBisonOutput, CreateBisonError> {
    tracing::info!("POST /bison body: {:?}", data);
    Ok(CreateBisonOutput::builder()
        .version(version())
        .id("someId")
        .name("SomeName")
        .build())
}
pub async fn stubbed_list_bison(data: ListBisonInput) -> Result<ListBisonOutput, ListBisonError> {
    tracing::info!("GET /bison/{}", data.herd);
    Ok(ListBisonOutput::builder().max(8).version(version()).build())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app: Router = OperationRegistryBuilder::default()
        .create_bison(stubbed_create_bison)
        .list_bison(stubbed_list_bison)
        .build()
        .expect("Failed to build wrapper service")
        .into();

    let app = app.layer(tower_tracer());

    run_in_lambda(app.into_make_lambda_service()).await?;
    Ok(())
}
