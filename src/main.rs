#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{NewPostHandler, Post};
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get a connection from the pool");

    match web::block(move || posts.load::<Post>(&conn)).await {
        Ok(app_data) => {
            println!("{:?}", app_data);
            return HttpResponse::Ok().body(format!("{:?}", app_data));
        }
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[post("/new-post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("couldn't get a connection from the pool");

    match web::block(move || Post::create_post(&conn, &item)).await {
        Ok(app_data) => {
            println!("{:?}", app_data);
            return HttpResponse::Ok().body(format!("{:?}", app_data));
        }
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = ConnectionManager::<PgConnection>::new(db_url); // this handles the connection to the database
    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(new_post)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 9900))?
    .run()
    .await
}
