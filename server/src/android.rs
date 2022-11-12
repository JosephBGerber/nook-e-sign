use actix_web::dev::HttpServiceFactory;
use actix_web::{get, post, web, Responder, HttpResponse};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

use crate::error::*;

#[derive(Debug, Serialize, Deserialize)]
struct Id {
    id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Charge {
    charge: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    id: i64,
    library_id: i64,
    charge: i64,
    image_hash: Option<String>,
}

#[derive(Debug, Serialize)]
struct Library {
    id: i64,
    name: String,
}

#[post("/device/{id}/charge")]
async fn post_charge(pool: web::Data<PgPool>, web::Path(id): web::Path<i64>, device: web::Query<Charge>) -> Result<impl Responder, Error> {
    let id = sqlx::query_scalar!(
        "SELECT id FROM device WHERE id = $1",
        id as i64
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
async fn get_image(pool: web::Data<PgPool>, web::Path(id): web::Path<i64>) -> Result<impl Responder, Error> {
    let device = sqlx::query!(
        "SELECT image FROM device WHERE id = $1",
        id as i64
    )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    if let Some(device) = device {
        if let Some(image) = device.image {
            Ok(web::Bytes::from(image))
        } else {
            Ok(web::Bytes::from(crate::NOT_FOUND_IMAGE.as_slice()))
        }
    } else {
        Err(Error::Forbidden)
    }
}


#[get("/device/{id}")]
async fn get_device(pool: web::Data<PgPool>, web::Path(id): web::Path<i64>) -> Result<impl Responder, Error> {
    let device = sqlx::query_as!(
        Device,
        "SELECT id, library_id, charge, md5(image) as image_hash FROM device WHERE id = $1",
        id as i64
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
async fn post_library_device(pool: web::Data<PgPool>, web::Path(id): web::Path<i64>) -> Result<impl Responder, Error> {
    let library = sqlx::query_as!(
            Library,
            "SELECT id, name FROM library WHERE id = $1",
            id as i64
        )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    if let Some(library) = library {
        let id = sqlx::query!(
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

pub fn setup() -> impl HttpServiceFactory {
    web::scope("android")
        .service(get_device)
        .service(get_image)
        .service(get_library_find_by_name)
        .service(post_charge)
        .service(post_library_device)
}