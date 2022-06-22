#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;
use tera::Tera;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{NewPostHandler, Post};
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("couldn't get a connection from the pool");

    match web::block(move || posts.load::<Post>(&conn)).await {
        Ok(data) => {
            // println!("{:?}", data);
            let data = data.unwrap();

            let mut ctx = tera::Context::new();
            ctx.insert("posts", &data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("index.html", &ctx).unwrap())
        }
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>,
) -> impl Responder {

    let conn = pool.get().expect("couldn't get a connection from the pool");

    let url_slug = blog_slug.into_inner();

    match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&conn)).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            let mut ctx = tera::Context::new();
            println!("{:?}", data);
            ctx.insert("post", data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("posts.html", &ctx).unwrap())

            // return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Err(_err) => HttpResponse::Ok().body("Error while getting the post"),
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
    let port = env::var("PORT").expect("PORT must be set");
    let port : u16 = port.parse().unwrap();

    let connection = ConnectionManager::<PgConnection>::new(db_url); // this handles the connection to the database
    let pool = Pool::builder()
        .build(connection)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .service(index)
            .service(new_post)
            .service(get_post)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
