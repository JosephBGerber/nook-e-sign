use crate::error::*;

use actix_multipart::Multipart;
use actix_session::{CookieSession, Session};
use actix_web::dev::HttpServiceFactory;
use actix_web::FromRequest;
use actix_web::{get, post, delete, web, Responder, HttpResponse};
use sqlx::PgPool;
use futures::stream::StreamExt;
use image;
use image::ImageOutputFormat;
use image::imageops::FilterType;
use serde::{Serialize, Deserialize};

#[get("/device/{id}/image")]
async fn get_image(pool: web::Data<PgPool>, web::Path(id): web::Path<i64>) -> Result<impl Responder, Error> {
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
            Ok(web::Bytes::from(crate::NOT_FOUND_IMAGE.as_slice()))
        }
    } else {
        Err(Error::Forbidden)
    }
}

#[post("/device/{id}/image")]
async fn post_image(pool: web::Data<PgPool>, session: Session, web::Path(id): web::Path<i64>, mut multipart: Multipart) -> Result<impl Responder, Error> {
    let library_id = session.get::<i64>("library_id")
        .map_err(map_session_error)?
        .ok_or(Error::Forbidden)?;

    let id = sqlx::query_scalar!(
        "SELECT (id) FROM device WHERE id = $1 AND library_id = $2",
        id,
        library_id
    )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;


    let mut result = false;

    if let Some(id) = id {
        while let Some(field) = multipart.next().await {
            let mut field = field.map_err(map_multipart_error)?;

            let name = field
                .content_disposition()
                .ok_or(Error::BadRequest)?
                .get_name()
                .ok_or(Error::BadRequest)?
                .to_string();

            if name == "image" {
                let mut content = Vec::new();

                while let Some(chunk) = field.next().await {
                    content.append(&mut chunk.map_err(map_multipart_error)?.to_vec());
                }

                let img = image::load_from_memory(&content)
                    .map_err(map_image_error)?;
                let img = img.resize_exact(600, 800, FilterType::Lanczos3);
                let mut bytes = Vec::new();
                img.write_to(&mut bytes, ImageOutputFormat::Jpeg(32))
                    .map_err(map_image_error)?;

                sqlx::query("UPDATE device SET image = $1 WHERE id = $2")
                    .bind(&bytes)
                    .bind(id)
                    .execute(pool.get_ref())
                    .await
                    .map_err(map_sqlx_error)?;

                result = true;
            }
        }
    }

    return if result {
        Ok(HttpResponse::Ok())
    } else {
        Err(Error::BadRequest)
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    id: i64,
    library_id: i64,
    charge: i64,
    image_hash: Option<String>,
}

#[get("/device/{id}")]
async fn get_device(pool: web::Data<PgPool>, session: Session, web::Path(id): web::Path<i64>) -> Result<impl Responder, Error> {
    let library_id = session.get::<i64>("library_id")
        .map_err(map_session_error)?
        .ok_or(Error::Forbidden)?;

    let device = sqlx::query_as!(
        Device,
        "SELECT id, library_id, charge, md5(image) as image_hash FROM device WHERE id = $1 and library_id = $2",
        id,
        library_id
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

#[delete("/device/{id}")]
async fn delete_device(pool: web::Data<PgPool>, session: Session, web::Path(id): web::Path<i64>) -> Result<impl Responder, Error> {
    let library_id = session.get::<i64>("library_id")
        .map_err(map_session_error)?
        .ok_or(Error::Forbidden)?;

    let result = sqlx::query("DELETE FROM device WHERE id = $1 AND library_id = $2")
        .bind(id)
        .bind(library_id)
        .execute(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    Ok(format!("{}", result.rows_affected()))
}

#[get("/device")]
async fn get_devices(pool: web::Data<PgPool>, session: Session) -> Result<impl Responder, Error> {
    let library_id = session.get::<i64>("library_id")
        .map_err(map_session_error)?
        .ok_or(Error::Forbidden)?;

    let devices = sqlx::query_as!(
        Device,
        "SELECT id, library_id, charge, md5(image) as image_hash FROM device WHERE library_id = $1",
        library_id
    )
        .fetch_all(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    Ok(actix_web::web::Json(devices))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    name: String,
    password: String,
}

#[get("/login")]
pub async fn login(pool: web::Data<PgPool>, session: Session, login: web::Query<Login>) -> Result<impl Responder, Error> {
    let library = sqlx::query!(
        "SELECT id, name, password FROM library WHERE name = $1",
        &login.name
    )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(map_sqlx_error)?;

    if let Some(library) = library {
        if library.password == login.password {
            session.set("library_id", library.id)
                .map_err(map_session_error)?;
            Ok(HttpResponse::Ok())
        } else {
            Err(Error::BadRequest)
        }
    } else {
        Err(Error::BadRequest)
    }
}

#[get("/logout")]
pub async fn logout(session: Session) -> Result<impl Responder, Error> {
    session.clear();
    Ok(HttpResponse::Ok())
}

pub fn setup() -> impl HttpServiceFactory {
    web::scope("client")
        .app_data(web::PayloadConfig::new(1_000_000 * 500))
        .app_data(web::Bytes::configure(|cfg| {
            cfg.limit(1_000_000 * 500) // 500 MB
        }))
        .wrap(CookieSession::private(&[0; 32]).secure(false))
        .service(get_image)
        .service(post_image)
        .service(get_device)
        .service(delete_device)
        .service(get_devices)
        .service(login)
        .service(logout)
}