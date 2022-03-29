extern crate log;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use anyhow::Result as AnyhowResult;
use dotenv;
use sea_orm::{ConnectOptions, Database};
use std::{convert::Infallible, env, time::Duration};

mod core;
mod dto;
mod entity;
mod handler;
mod repository;

use crate::core::AppState;
use handler::user::*;

#[actix_web::main]
async fn main() -> AnyhowResult<()> {
    dotenv::from_filename(".env").ok();
    dotenv::from_filename(".env.local").ok();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

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

    log::info!("using database at: {}", &db_url);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(db_max_connections)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .sqlx_logging(true)
        .idle_timeout(Duration::from_secs(8));

    let db_pool = Database::connect(opt).await?;
    let app_state = web::Data::new(AppState::new(db_pool));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .route(
                "/",
                web::get().to(|| async {
                    Ok::<_, Infallible>(HttpResponse::Ok().body(r#"Learn Rust project"#))
                }),
            )
            .service(get_user_by_id)
            .default_service(web::route().to(HttpResponse::MethodNotAllowed))
    })
    .bind((host, port))?;

    log::info!("Starting server");
    server.run().await?;

    Ok(())
}
