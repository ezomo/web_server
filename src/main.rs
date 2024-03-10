use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use actix_files as fs;
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use thiserror::Error;




impl ResponseError for MyError {}




#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConncectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {


    let html = IndexTemplate {  };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table `todo`.");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .data(pool.clone())
    })
    .bind("0.0.0.0:80")?
    .run()
    .await?;
    Ok(())
}
