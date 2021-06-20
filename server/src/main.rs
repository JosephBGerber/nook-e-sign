#![feature(proc_macro_hygiene)]

mod android;
mod client;
mod error;

use actix_files::Files;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use image;
use image::ImageOutputFormat;
use sqlx::postgres::PgPoolOptions;
use actix_web::middleware::Logger;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref NOT_FOUND_IMAGE: Vec<u8> = {
        let img = image::open("not_found.png").unwrap();
        let mut bytes = Vec::new();
        img.write_to(&mut bytes, ImageOutputFormat::Jpeg(32)).unwrap();
        bytes
    };
}


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must defined")).await?;

    let port = if cfg!(debug_assertions) {
        8080
    } else {
        80
    };

    let dist = if cfg!(debug_assertions) {
        "../client/dist"
    } else {
        "./dist"
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(android::setup())
            .service(client::setup())
            .service(Files::new("/", dist)
                .index_file("index.html"))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await?;

    Ok(())
}
