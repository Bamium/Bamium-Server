use std::{time::Duration};

use std::{sync::Arc};


use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, HttpRequest, http::header, web::Data};

use juniper_actix::{playground_handler, graphql_handler, subscriptions::subscriptions_handler};
use juniper_graphql_ws::ConnectionConfig;
use mongodb::Client;

use crate::models::Databases;
use crate::{models::Context};

mod schema;
mod auth;

// Webserver handlers and initialization

const PORT: &str = "8080";

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", Some("/subscriptions")).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<schema::Schema>,
    databases: web::Data<Databases>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &Context { db: databases.get_ref().clone(), userid: None }, req, payload).await
}

async fn subscriptions(
    req: HttpRequest,
    stream: web::Payload,
    schema: web::Data<schema::Schema>,
    databases: web::Data<Databases>,
) -> Result<HttpResponse, Error> {
    let schema = schema.into_inner();
    let config = ConnectionConfig::new(Context { db: databases.get_ref().clone(), userid: None });
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions_handler(req, stream, schema, config).await
}

pub async fn init_webserver(dbclient: Client) {
    let schema = Arc::new(schema::schema());
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(Databases { db: dbclient.clone() }))
            .wrap(Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600),
        )
        .wrap(middleware::Compress::default())
        .wrap(middleware::Logger::default())
        // Routes
        // Playground endpoint
        .service(
            web::resource("/playground")
                .route(web::get().to(playground)),
        )
        // GraphQL endpoint
        .service(
            web::resource("/graphql")
                .route(web::post().to(graphql))
                .route(web::get().to(graphql)),
        )
        // Subscriptions endpoint
        .service(
            web::resource("/subscriptions")
                .route(web::post().to(subscriptions)),
        )
        // Set default endpoint to playground
        .default_service(web::to(|| async {
            HttpResponse::Found()
                .append_header(("LOCATION", "/playground"))
                .finish()
        }))
    })
    .bind(format!("127.0.0.1:{}", PORT))
    .unwrap()
    .run()
    .await
    .unwrap();
}