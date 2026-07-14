mod api;
mod db;
mod middleware;
mod state;

use crate::api::api;
use crate::middleware::middleware;
use crate::state::{Duration, State};
use actix_files::Files;
use actix_web::web::{Data, Redirect};
use actix_web::{App, HttpServer, Responder, get};
use std::env::current_dir;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = current_dir()?;
    println!("DB dir: {:?}", path);

    let state = Data::new(State {
        selected: Mutex::new(None),
        duration: Mutex::new(Duration::new(10).unwrap()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .configure(middleware)
            .configure(api)
            .service(Files::new("/", "./static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

#[get("/")]
async fn index(_: Data<State>) -> impl Responder {
    Redirect::to("/control.html").permanent()
}
