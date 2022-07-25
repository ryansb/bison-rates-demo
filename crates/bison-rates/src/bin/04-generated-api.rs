use aws_smithy_http_server::Router;
use lambda_http::{run as run_in_lambda, Error};

use sdk_server_bison_rates::operation_registry::OperationRegistryBuilder;

use bison_rates::meta::tower_tracer;
use bison_rates::storage::{create_bison, list_bison};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app: Router = OperationRegistryBuilder::default()
        .create_bison(create_bison)
        .list_bison(list_bison)
        .build()
        .expect("Failed to build wrapper service")
        .into();

    let app = app.layer(tower_tracer());

    run_in_lambda(app.into_make_lambda_service()).await?;
    Ok(())
}
