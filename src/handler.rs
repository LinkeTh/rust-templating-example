use crate::error::{AppError, ErrorTemplate};
use crate::{db, Book};
use askama::Template;
use axum::extract::{Path, State};
use axum::Form;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
// use axum::debug_handler;

#[derive(Template)]
#[template(path = "book/list_books.html")]
pub struct BookListTemplate {
    books: Vec<Book>,
}
#[derive(Template)]
#[template(path = "book/new_book.html")]
pub struct NewBookTemplate {}

#[derive(Template)]
#[template(path = "book/edit_book.html")]
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
#[template(path = "template.html")]
pub struct BaseTemplate {}

pub async fn welcome_handler(State(db_pool): State<PgPool>) -> Result<BaseTemplate, AppError> {
    db::test_db(db_pool.clone()).await?;

    Ok(BaseTemplate {})
}
// #[debug_handler]
pub async fn books_list_handler(
    State(db_pool): State<PgPool>,
) -> Result<BookListTemplate, AppError> {
    let books = db::find_books(db_pool.clone()).await?;

    Ok(BookListTemplate { books: books })
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
) -> Result<(), AppError> {
    let result = db::delete_book(&book_id, db_pool.clone()).await;

    match result {
        Ok(r) => {
            println!("{:?} row(s) deleted", r.rows_affected());
        }
        Err(e) => {
            println!("Error deleting {} row: {}", book_id, e);
        }
    }

    Ok(())
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
) -> Result<BookListTemplate, AppError> {
    let result = db::update_book(&book_id, db_pool.clone(), body).await;

    match result {
        Ok(r) => {
            println!("{:?} row(s) updated", r.rows_affected());
        }
        Err(e) => {
            println!("Error updating {} row: {}", book_id, e);
        }
    }

    books_list_handler(State(db_pool)).await
}

// #[debug_handler]
pub async fn create_book_handler(
    State(db_pool): State<PgPool>,
    Form(body): Form<BookRequest>,
) -> Result<BookListTemplate, AppError> {
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
    books_list_handler(State(db_pool)).await
}
