use crate::model::post::Post;
use std::sync::Arc;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

use leptos::{logging::log, *};
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use uuid::Uuid;

pub struct DBError {}

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    let pool: Arc<Pool<Sqlite>> =
        extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;

    let id = id.unwrap_or(Uuid::new_v4().to_string());
    sqlx::query("INSERT INTO post VALUES ($1, $2, $3, $4, $5)")
        .bind(&id)
        .bind(&dt)
        .bind(&image_url)
        .bind(&title)
        .bind(&text)
        .execute(&*pool)
        .await?;

    Ok(id)
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    log!("get_post {:?}", &id);
    let pool: Arc<Pool<Sqlite>> =
        extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;
    let res: Post = sqlx::query_as("SELECT * FROM post WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|_| ServerFnError::ServerError("error getting post".to_owned()))?;

    Ok(res)
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    log!(
        "get_previews {:?}, {:?}, {}, {}",
        oldest,
        newest,
        preview_length,
        page_size
    );
    let pool: Arc<Pool<Sqlite>> =
        extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;
    let res: Vec<Post> = sqlx::query_as(
        "SELECT
            id, dt, image_url, title,
            CASE
                WHEN LENGTH(text) > $1 THEN SUBSTR(text, $1, ?) || '...'
                ELSE text
            END AS text
        FROM post
        ORDER BY dt DESC
        LIMIT $2",
    )
    .bind(preview_length)
    .bind(page_size)
    .fetch_all(&*pool)
    .await?;

    // Err(ServerFnError::ServerError("forced error".to_string()))
    Ok(res)
}
