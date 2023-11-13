use anyhow::Context;
use axum::routing::{delete, put};
use axum::{routing::get, routing::post, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};
use tower_http::cors::CorsLayer;

mod db;
mod error;
mod handler;

#[derive(Clone, Debug, Serialize, FromRow, Default)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub language: String,
    pub pages: i32,
    pub added_at: DateTime<Utc>,
}
type DBPool = Pool<Postgres>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = db::create_pool()
        .await
        .expect("failed to create database pool");

    let all_routes = Router::new()
        .merge(build_welcome_route(db_pool.clone()))
        .merge(build_book_routes_with_db(db_pool.clone()))
        .merge(build_book_routes())
        .fallback(handler::handler_404)
        .layer(CorsLayer::permissive());

    println!("Started on port 8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(all_routes.into_make_service())
        .await
        .context("failed to start server")
}

fn build_welcome_route(db_pool: DBPool) -> Router {
    Router::new()
        .route("/", get(handler::welcome_handler))
        .with_state(db_pool)
}

fn build_book_routes_with_db(db_pool: DBPool) -> Router {
    Router::new()
        .route("/books/new", post(handler::create_book_handler))
        .route("/books/edit/:book_id", get(handler::edit_book_handler))
        .route("/books/edit/:book_id", put(handler::do_edit_book_handler))
        .route(
            "/books/delete/:book_id",
            delete(handler::delete_book_handler),
        )
        .route("/books/list", get(handler::books_list_handler))
        .with_state(db_pool)
}

fn build_book_routes() -> Router {
    Router::new().route("/books/new", get(handler::new_book_handler))
}
