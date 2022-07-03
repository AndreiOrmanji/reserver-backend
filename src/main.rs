extern crate log;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use anyhow::Result as AnyhowResult;
use dotenv;
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use std::{convert::Infallible, env, time::Duration};

mod core;
mod dto;
mod handler;
mod repository;

use crate::core::AppState;
use handler::{user::*, work_desk::*};

#[actix_web::main]
async fn main() -> AnyhowResult<()> {
    dotenv::from_filename(".env").ok();
    dotenv::from_filename(".env.local").ok();

    tracing_subscriber::fmt::init();

    println!(
        "{:?}",
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file")
    );

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
        .expect("DATABASE_MAX_CONNECTIONS is not set in .env file")
        .parse::<u32>()
        .unwrap();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .expect("PORT should be a u16");
    let server_url = format!("{}:{}", host, port);

    log::info!("using database at: {}", &db_url);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(db_max_connections)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .sqlx_logging(true)
        .idle_timeout(Duration::from_secs(8));

    let db_pool = Database::connect(opt).await?;

    Migrator::up(&db_pool, None).await?;

    let app_state = web::Data::new(AppState::new(db_pool));

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .route(
                "/",
                web::get().to(|| async {
                    Ok::<_, Infallible>(HttpResponse::Ok().body(r#"Learn Rust project"#))
                }),
            )
            .service(get_floor_by_id_of_center_by_id)
            .service(get_user_by_id)
            .default_service(web::route().to(HttpResponse::MethodNotAllowed))
    });

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {}", server_url);
    server.run().await?;

    Ok(())
}
