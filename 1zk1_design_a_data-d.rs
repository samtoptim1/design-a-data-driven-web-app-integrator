Rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::error::Error;

// Define a struct to hold our data
#[derive(Debug, Deserialize, Serialize)]
struct Data {
    id: i32,
    name: String,
    value: String,
}

// Define a struct to hold our database connection
struct Database {
    pool: PgPool,
}

impl Database {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let pool = PgPool::new("postgresql://user:password@localhost/database").await?;
        Ok(Database { pool })
    }

    async fn get_data(&self) -> Result<Vec<Data>, Box<dyn Error>> {
        let rows = sqlx::query("SELECT * FROM data")
            .fetch_all(&self.pool)
            .await?;

        let data: Vec<Data> = rows
            .into_iter()
            .map(|row| Data {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                value: row.try_get(2)?,
            })
            .collect();

        Ok(data)
    }
}

async fn index(db: web::Data<Database>) -> impl Responder {
    let data = db.get_data().await.unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&data).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new().await.unwrap();
    let data_db = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(data_db.clone())
            .service(web::resource("/data").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}