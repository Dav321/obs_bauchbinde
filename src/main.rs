use actix_web::web::{Data, Redirect};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use sqlx::{Executor, SqlitePool};
use std::env::current_dir;

pub struct State {
    pool: SqlitePool,
    selected: Option<usize>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = current_dir()?;
    println!("DB dir: {:?}", path);
    let pool = SqlitePool::connect("sqlite:bauchbinde.db?mode=rwc").await.unwrap();
    pool.execute(include_str!("sql/migrate.sql")).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                pool: pool.clone(),
                selected: None,
            }))
            .service(index)
            .service(control)
            .service(view)
    })
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}

#[get("/")]
async fn index(_: Data<State>) -> impl Responder {
    Redirect::to("/control").permanent()
}

#[get("/control")]
async fn control(state: Data<State>) -> impl Responder {
    HttpResponse::Ok().body("Control away!")
}

#[get("/view")]
async fn view(state: Data<State>) -> impl Responder {
    HttpResponse::Ok().body("great view!")
}