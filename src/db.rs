use dotenv_codegen::dotenv;
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::PgPool;

use crate::handler::BookRequest;
use crate::{Book, DBPool};

pub async fn create_book(db_pool: PgPool, new_book: Book) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO BOOK (id, name, author, language, pages, added_at) VALUES ($1, $2, $3, $4, $5, $6)",
        &new_book.id,
        &new_book.name,
        &new_book.author,
        &new_book.language,
        &new_book.pages,
        &new_book.added_at)
        .execute(&db_pool)
        .await
}

pub async fn test_db(db_pool: PgPool) -> Result<(), sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT $1")
        .bind("success")
        .fetch_one(&db_pool)
        .await?;

    println!("db connection {:?}", row.0);
    assert_eq!(row.0, "success");

    Ok(())
}

pub async fn find_books(db_pool: PgPool) -> Result<Vec<Book>, sqlx::Error> {
    sqlx::query_as!(Book, "SELECT * FROM BOOK")
        .fetch_all(&db_pool)
        .await
}

pub async fn delete_book(book_id: &str, db_pool: PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM book WHERE id = $1", book_id)
        .execute(&db_pool)
        .await
}

pub async fn find_book(book_id: &str, db_pool: PgPool) -> Result<Book, sqlx::Error> {
    sqlx::query_as!(Book, "SELECT * FROM BOOK WHERE id = $1", book_id)
        .fetch_one(&db_pool)
        .await
}

pub async fn update_book(
    book_id: &str,
    db_pool: PgPool,
    body: BookRequest,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE BOOK SET name = $1, author = $2, language = $3, pages = $4 WHERE id = $5 ",
        &body.name,
        &body.author,
        &body.language,
        &body.pages,
        book_id
    )
    .execute(&db_pool)
    .await
}

pub async fn create_pool() -> Result<DBPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn my_test() {
        let result = create_pool().await;
        match result {
            Ok(_r) => {
                assert!(true);
            }
            Err(_e) => {
                assert!(false);
            }
        }
    }
}
