use lambda_http::{
    run,
    tower::{ServiceBuilder, ServiceExt},
    Error, Request, Response,
};

use bison_rates::meta::{tower_tracer, version_header};

async fn handler(_req: Request) -> Result<Response<String>, Error> {
    Ok(Response::builder().status(404).body("{}".into()).unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = ServiceBuilder::new()
        .layer(tower_tracer())
        .service_fn(handler)
        .map_response(version_header);

    run(service).await?;
    Ok(())
}
