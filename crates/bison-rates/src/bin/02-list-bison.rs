use lambda_http::{
    run,
    tower::{ServiceBuilder, ServiceExt},
    Error, Request, Response,
};

use bison_rates::meta::{tower_tracer, version_header};
use bison_rates::responses::BisonListResponse;

async fn handler(req: Request) -> Result<Response<String>, Error> {
    tracing::info!({ path = req.uri().path(), query = req.uri().query() }, "GET /herds");
    Ok(Response::new(
        serde_json::to_string(&BisonListResponse {
            name: "default".to_string(),
            herd: None,
        })
        .unwrap(),
    ))
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
