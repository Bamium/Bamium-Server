use std::{time::Duration};

use actix_cors::Cors;
use actix_web::{get, post};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, http::header, web::Data};

use juniper_actix::{playground_handler, graphql_handler, subscriptions::subscriptions_handler};
use juniper_graphql_ws::ConnectionConfig;
use mongodb::Client;

use crate::models::Databases;
use crate::{models::Context};

mod schema;
mod auth;


#[get("/playground")]
async fn playground() -> Result<HttpResponse, Error> {
    println!("Playground");
    playground_handler("/graphql", Some("/subscriptions")).await
}

#[post("/graphql")]
async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<schema::Schema>,
    databases: web::Data<Databases>,
) -> Result<HttpResponse, Error> {
    println!("gql");
    graphql_handler(schema.get_ref(), &Context { db: databases.get_ref().clone(), userid: Some(1) }, req, payload).await
}

#[get("/subscriptions")]
async fn subscriptions(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    schema: web::Data<schema::Schema>,
    databases: web::Data<Databases>,
) -> Result<HttpResponse, Error> {
    println!("gqlsub");
    let config = ConnectionConfig::new(Context { db: databases.get_ref().clone(), userid: Some(1) });
    let config = config.with_keep_alive_interval(Duration::from_secs(15));
    let schema = schema.into_inner();

    subscriptions_handler(req, stream, schema, config).await
}

pub async fn init_webserver(dbs: Databases) {
    HttpServer::new(move || {
        println!("init");
        App::new()
            .app_data(Data::new(schema::schema()))
            .app_data(Data::new(dbs.clone()))
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
        .service(graphql)
        .service(playground)
        .service(subscriptions)
        .default_service(web::to(|| async {
            HttpResponse::Found()
                .append_header(("LOCATION", "/playground"))
                .finish()
        }))
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}