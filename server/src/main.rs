#![feature(proc_macro_hygiene)]

mod error;
mod client;

use error::*;

use actix_files::Files;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use dotenv::dotenv;
use image;
use image::ImageOutputFormat;
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
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

#[derive(Debug, Serialize, Deserialize)]
struct Id {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Charge {
    charge: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    id: i32,
    library_id: i32,
    charge: i32,
    image_hash: Option<String>,
}

#[post("/device/{id}/charge")]
async fn post_charge(pool: web::Data<PgPool>, web::Path(id): web::Path<i32>, device: web::Query<Charge>) -> Result<impl Responder, Error> {
    let id = sqlx::query_scalar!(
        "SELECT (id) FROM device WHERE id = $1",
        id
    )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;


    return if let Some(id) = id {
        if device.charge < 0 || device.charge > 100 {
            Err(Error::BadRequest)
        } else {
            sqlx::query("UPDATE device SET charge = $1 WHERE id = $2")
                .bind(device.charge)
                .bind(id)
                .execute(pool.get_ref())
                .await
                .map_err(map_sqlx_error)?;
            Ok(HttpResponse::Ok())
        }
    } else {
        Err(Error::BadRequest)
    };
}

#[get("/device/{id}/image")]
async fn get_image(pool: web::Data<PgPool>, web::Path(id): web::Path<i32>) -> Result<impl Responder, Error> {
    let device = sqlx::query!(
        "SELECT (image) FROM device WHERE id = $1",
        id
    )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    if let Some(device) = device {
        if let Some(image) = device.image {
            Ok(web::Bytes::from(image))
        } else {
            Ok(web::Bytes::from(NOT_FOUND_IMAGE.as_slice()))
        }
    } else {
        Err(Error::Forbidden)
    }
}


#[get("/device/{id}")]
async fn get_device(pool: web::Data<PgPool>, web::Path(id): web::Path<i32>) -> Result<impl Responder, Error> {
    let device = sqlx::query_as!(
        Device,
        "SELECT id, library_id, charge, md5(image) as image_hash FROM device WHERE id = $1",
        id
    )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    if let Some(device) = device {
        Ok(actix_web::web::Json(device))
    } else {
        Err(Error::Forbidden)
    }
}

#[derive(Debug, Serialize)]
struct Library {
    id: i32,
    name: String,
}

#[get("/library/findByName/{name}")]
async fn get_library_find_by_name(pool: web::Data<PgPool>, web::Path(name): web::Path<String>) -> Result<impl Responder, Error> {
    let library = sqlx::query_as!(
            Library,
            "SELECT id, name FROM library WHERE name ILIKE $1",
            name
        )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    Ok(actix_web::web::Json(library))
}

#[post("/library/{id}/device")]
async fn post_library_device(pool: web::Data<PgPool>, web::Path(id): web::Path<i32>) -> Result<impl Responder, Error> {
    let library = sqlx::query_as!(
            Library,
            "SELECT id, name FROM library WHERE id = $1",
            id
        )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    if let Some(library) = library {
        let id = sqlx::query_as!(
                Id,
                "INSERT INTO device(library_id, charge) VALUES ($1, 100) RETURNING id",
                library.id
            )
            .fetch_one(pool.get_ref())
            .await
            .map_err(map_sqlx_error)?;

        let device = sqlx::query_as!(
                Device,
                "SELECT id, library_id, charge, md5(image) as image_hash FROM device WHERE id = $1",
                id.id
            )
            .fetch_optional(pool.get_ref())
            .await
            .map_err(map_sqlx_error)?;

        Ok(actix_web::web::Json(device))
    } else {
        Err(Error::Forbidden)
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must defined")).await?;


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(client::setup())
            .service(post_charge)
            .service(get_device)
            .service(get_library_find_by_name)
            .service(post_library_device)
            .service(Files::new("/", "../client/dist")
                .index_file("index.html"))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    Ok(())
}
