use actix_files::Files;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use env_logger;
use serde_json;

mod schema;
use crate::schema::{create_schema, Database, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8088/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    app_data: web::Data<AppData>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&app_data.schema, &app_data.db);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[derive(Debug)]
struct AppData {
    schema: Schema,
    db: Database,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = web::Data::new(AppData {
        schema: create_schema(),
        db: Database {
            todos: "Todos".to_owned(),
        },
    });
    println!("{:?}", schema);

    HttpServer::new(move || {
        App::new()
            .app_data(schema.clone())
            .wrap(middleware::Logger::default())
            .route("/graphql", web::post().to(graphql))
            .route("/graphiql", web::get().to(graphiql))
            .service(Files::new("/", "../frontend"))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
