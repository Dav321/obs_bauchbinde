mod api;
mod db;
mod frontend;
mod state;

use crate::api::api;
use crate::frontend::frontend;
use crate::state::{Duration, State};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::env::current_dir;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print_header();

    let state = Data::new(State {
        selected: Mutex::new(None),
        duration: Mutex::new(Duration::new(10).unwrap()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(frontend)
            .configure(api)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

fn print_header() {
    println!("Bauchbinde v{}", env!("CARGO_PKG_VERSION"));
    println!("---");
    println!("control: http://localhost:5000/control.html");
    println!("view: http://localhost:5000/view.html");
    match current_dir() {
        Ok(path) => println!("DB dir: {:?}", path),
        Err(e) => println!("{e}"),
    }
    println!("---");
}
