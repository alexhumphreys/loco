use async_trait::async_trait;
use axum::Extension;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;

use std::sync::Arc;

use aide::{
    axum::ApiRouter,
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};

use crate::controllers;

pub struct AideInitializer;

#[async_trait]
impl Initializer for AideInitializer {
    fn name(&self) -> String {
        "aide".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        aide::gen::on_error(|error| {
            println!("{error}");
        });

        aide::gen::extract_schemas(true);

        let mut api = OpenApi::default();

        let router_with_aide = ApiRouter::new()
            .nest_api_service("/app", router)
            .nest_api_service("/docs", controllers::docs::docs_routes())
            .finish_api_with(&mut api, api_docs)
            .layer(Extension(Arc::new(api))); // Arc is very important here or you will face massive memory and performance issues

        Ok(router_with_aide)
    }
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Aide axum Open API")
        .summary("An example Todo application")
        .description("Longer bit of text that could be read from a file")
        .tag(Tag {
            name: "todo".into(),
            description: Some("Todo Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
}
