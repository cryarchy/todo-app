use std::env;
use std::sync::{Arc, Mutex};

use actix_files::Files;
use actix_session::{CookieSession, Session};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use env_logger;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use listenfd::ListenFd;
use mongodb::Client;
use serde_json;

use backend::{
    app_context::AppContext,
    schema::{create_schema, Schema},
    shared::Collections,
    user::User,
};

async fn graphiql() -> HttpResponse {
    let app_domain = env::var("APP_DOMAIN").expect("Expected APP_DOMAIN to be set in env!");
    let html = graphiql_source(&format!("http://{}/graphql", app_domain));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    session: Session,
    app_data: web::Data<AppData>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // keep lock scope small
    {
        let mut ctx_user = app_data.context.session.lock().unwrap();
        *ctx_user = session.get::<User>("user")?;
    }

    let user = web::block(move || {
        let res = data.execute(&app_data.schema, &app_data.context);
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
    context: AppContext,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let app_domain = env::var("APP_DOMAIN").expect("Expected APP_DOMAIN to be set in env!");
    let db_uri = env::var("DB_URI").expect("Expected DB_URI to be set in env!");
    let db_name = env::var("DB_NAME").expect("Expected DB_NAME to be set in env!");

    let client = Client::with_uri_str(&db_uri).expect("Failed to initialize database!");
    let db = client.database(&db_name);

    let app_data = web::Data::new(AppData {
        schema: create_schema(),
        context: AppContext {
            db: db,
            session: Arc::new(Mutex::new(None)),
            collections: Collections::new(),
        },
    });

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::default())
            .route("/graphql", web::post().to(graphql))
            .route("/graphiql", web::get().to(graphiql))
            .service(Files::new("/", "../frontend"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(app_domain)?
    };

    server.run().await
}
