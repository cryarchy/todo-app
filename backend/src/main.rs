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
    user::User,
};

async fn graphiql() -> HttpResponse {
    let app_port = env::var("APP_PORT").unwrap();
    let app_host = env::var("APP_HOST").unwrap();
    let html = graphiql_source(&format!("http://{}:{}/graphql", app_host, app_port));
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

    let app_port: u32 = env::var("APP_PORT")
        .expect("Expected APP_PORT to be set in env!")
        .parse()
        .expect("Failed parsing APP_PORT to integer");
    let app_host = env::var("APP_HOST").expect("Expected APP_HOST to be set in env!");
    let db_host = env::var("DB_HOST").expect("Expected DB_HOST to be set in env!");
    let db_port: u16 = env::var("DB_PORT")
        .expect("Expected DB_PORT to be set in env!")
        .parse()
        .expect("Failed parsing APP_HOST to integer");
    let db_user = env::var("DB_USER").expect("Expected DB_USER to be set in env!");
    let db_pwd = env::var("DB_PWD").expect("Expected DB_PWD to be set in env!");
    let db_name = env::var("DB_NAME").expect("Expected DB_NAME to be set in env!");

    let client = Client::with_uri_str(&format!(
        "mongodb://{}:{}@{}:{}",
        db_user, db_pwd, db_host, db_port
    ))
    .expect("Failed to initialize database!");
    let db = client.database(&db_name);

    let app_data = web::Data::new(AppData {
        schema: create_schema(),
        context: AppContext {
            db: db,
            session: Arc::new(Mutex::new(None)),
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
        server.bind(format!("{}:{}", app_host, app_port))?
    };

    server.run().await
}
