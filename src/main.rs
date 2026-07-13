use actix_files::Files;
use actix_web::web::{Data, Redirect};
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use sqlx::{Executor, SqlitePool};
use std::env::current_dir;
use std::fs;
use std::path::Path;

pub struct State {
    pool: SqlitePool,
    selected: Option<usize>,
    duration: u8,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = current_dir()?;
    println!("DB dir: {:?}", path);
    let pool = SqlitePool::connect("sqlite:bauchbinde.db?mode=rwc")
        .await
        .unwrap();
    pool.execute(include_str!("sql/migrate.sql")).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                pool: pool.clone(),
                selected: None,
                duration: 10,
            }))
            .service(index)
            .service(view_css)
            .service(Files::new("/", "./static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

#[get("/")]
async fn index(_: Data<State>) -> impl Responder {
    Redirect::to("/view.html").permanent()
}

#[get("/css/view.css")]
async fn view_css(state: Data<State>) -> impl Responder {
    let duration = state.duration as f32;

    // Durations
    const OBJ_INTRO: f32 = 0.95;
    const TEXT_DELAY: f32 = 0.9;
    const TEXT_INTRO: f32 = 0.55;
    const FADE: f32 = 0.5;

    let obj = OBJ_INTRO / duration;
    let text_delay = TEXT_DELAY / duration;
    let text = text_delay + (TEXT_INTRO / duration);
    let fade = 1. - (FADE / duration);

    let css = fs::read_to_string(Path::new("./static/css/view.css"))
        .unwrap()
        .replace("/*obj*/0%", &format!("{:.4}%", obj * 100.))
        .replace("/*textd*/0%", &format!("{:.4}%", text_delay * 100.))
        .replace("/*text*/0%", &format!("{:.4}%", text * 100.))
        .replace("/*fade*/0%", &format!("{:.4}%", fade * 100.));

    HttpResponse::Ok().body(css)
}
