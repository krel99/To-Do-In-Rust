#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::config::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;

mod config;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};

mod database;
mod models;
mod schema;

mod json_serialization;
mod jwt;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
