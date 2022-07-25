use lambda_http::{
    run,
    tower::{ServiceBuilder, ServiceExt},
    Error, Request, Response,
};

use bison_rates::meta::{tower_tracer, version_header};

async fn handler(_req: Request) -> Result<Response<String>, Error> {
    Ok(Response::new("Success".into()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // the service will respond to many requests, serially. Attaching
    // caches/memoization as additional .layer(...) calls can preserve state
    // across invokes
    let service = ServiceBuilder::new()
        .layer(tower_tracer())
        .service_fn(handler)
        .map_response(version_header);

    run(service).await?;
    Ok(())
}
