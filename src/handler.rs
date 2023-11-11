use crate::error::{AppError, ErrorTemplate};
use crate::{db, Book};
use askama::Template;
use axum::extract::{Path, State};
use axum::response::Redirect;
use axum::Form;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// use axum::debug_handler;

#[derive(Template)]
#[template(path = "book/list.html")]
pub struct BookListTemplate {
    books: Vec<Book>,
}
#[derive(Template)]
#[template(path = "book/new.html")]
pub struct NewBookTemplate {}

#[derive(Template)]
#[template(path = "book/edit.html")]
pub struct EditBookTemplate {
    book: Book,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRequest {
    pub name: String,
    pub author: String,
    pub language: String,
    pub pages: i32,
}

#[derive(Template, Deserialize, Debug)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate {
    title: String,
    body: String,
}

// #[debug_handler]
pub async fn welcome_handler(State(db_pool): State<PgPool>) -> Result<WelcomeTemplate, AppError> {
    db::test_db(db_pool.clone()).await?;

    Ok(WelcomeTemplate {
        title: String::from("Welcome"),
        body: String::from("To The Bookstore!"),
    })
}
// #[debug_handler]
pub async fn books_list_handler(
    State(db_pool): State<PgPool>,
) -> Result<BookListTemplate, AppError> {
    let books = db::find_books(db_pool.clone()).await?;

    Ok(BookListTemplate { books })
}
// #[debug_handler]
pub async fn handler_404() -> Result<ErrorTemplate, AppError> {
    let message = "Page not found!";

    Ok(ErrorTemplate { message })
}

// #[debug_handler]
pub async fn delete_book_handler(
    Path(book_id): Path<String>,
    State(db_pool): State<PgPool>,
) -> Redirect {
    // ) -> Result<BookListTemplate, AppError> {
    let result = db::delete_book(&book_id, db_pool.clone()).await;

    match result {
        Ok(r) => {
            println!("{:?} row(s) deleted", r.rows_affected());
        }
        Err(e) => {
            println!("Error deleting {} row: {}", book_id, e);
        }
    }

    // books_list_handler(State(db_pool)).await
    Redirect::to("/books/list")
}

// #[debug_handler]
pub async fn new_book_handler() -> Result<NewBookTemplate, AppError> {
    Ok(NewBookTemplate {})
}

// #[debug_handler]
pub async fn edit_book_handler(
    Path(book_id): Path<String>,
    State(db_pool): State<PgPool>,
) -> Result<EditBookTemplate, AppError> {
    let book = db::find_book(&book_id, db_pool.clone()).await?;

    Ok(EditBookTemplate { book })
}

// #[debug_handler]
pub async fn do_edit_book_handler(
    Path(book_id): Path<String>,
    State(db_pool): State<PgPool>,
    Form(body): Form<BookRequest>,
) -> Redirect {
    // ) -> Result<BookListTemplate, AppError> {
    let result = db::update_book(&book_id, db_pool.clone(), body).await;

    match result {
        Ok(r) => {
            println!("{:?} row(s) updated", r.rows_affected());
        }
        Err(e) => {
            println!("Error updating {} row: {}", book_id, e);
        }
    }

    Redirect::to("/books/list")
    // books_list_handler(State(db_pool)).await
}

// #[debug_handler]
pub async fn create_book_handler(
    State(db_pool): State<PgPool>,
    Form(body): Form<BookRequest>,
) -> Redirect {
    let new_book = Book {
        id: Uuid::new_v4().to_string(),
        name: body.name,
        author: body.author,
        language: body.language,
        pages: body.pages,
        added_at: Utc::now(),
    };

    let result = db::create_book(db_pool.clone(), new_book).await;

    match result {
        Ok(r) => {
            println!("{:?} row(s) inserted", r.rows_affected());
        }
        Err(e) => {
            println!("Error inserting row: {}", e);
        }
    }
    Redirect::to("/books/list")
}
