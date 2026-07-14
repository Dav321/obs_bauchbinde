use crate::State;
use crate::db::{add_title, delete_title, edit_title, get_title};
use crate::state::Duration;
use actix_web::http::header;
use actix_web::web::{Data, ServiceConfig, scope};
use actix_web::{HttpResponse, Responder, delete, post};
use serde::Deserialize;

pub fn api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/titles")
            .service(select)
            .service(add)
            .service(delete)
            .service(edit),
    );
}

#[derive(Deserialize)]
struct SelectForm {
    pub preset: i64,
    pub duration: u8,
}

#[derive(Deserialize)]
struct TitleForm {
    pub name: String,
    pub label: String,
}

#[post("/")]
async fn add(title: actix_web::web::Form<TitleForm>) -> impl Responder {
    match add_title(&title.name, &title.label) {
        Ok(res) => {
            if res == 1 {
                HttpResponse::SeeOther()
                    .insert_header((header::LOCATION, "/control.html"))
                    .finish()
            } else {
                HttpResponse::BadRequest().body("Already Exists!")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{id}")]
async fn delete(id: actix_web::web::Path<i64>) -> impl Responder {
    match delete_title(*id) {
        Ok(res) => {
            if res == 1 {
                HttpResponse::SeeOther()
                    .insert_header((header::LOCATION, "/control.html"))
                    .finish()
            } else {
                HttpResponse::BadRequest().body(format!("No title with id {id}"))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/{id}")]
async fn edit(
    id: actix_web::web::Path<i64>,
    title: actix_web::web::Form<TitleForm>,
) -> impl Responder {
    match edit_title(*id, &title.name, &title.label) {
        Ok(res) => {
            if res == 1 {
                HttpResponse::SeeOther()
                    .insert_header((header::LOCATION, "/control.html"))
                    .finish()
            } else {
                HttpResponse::BadRequest().body(format!("No title with id {id} / Already Exists!"))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/select")]
async fn select(select: actix_web::web::Form<SelectForm>, state: Data<State>) -> impl Responder {
    let res = match get_title(select.preset) {
        Ok(res) => res,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if res.is_none() {
        return HttpResponse::BadRequest().body(format!("No title with id {}", select.preset));
    }

    *state.selected.lock().unwrap() = Some(select.preset);

    match Duration::new(select.duration) {
        None => {
            return HttpResponse::BadRequest()
                .body(format!("Invalid duration: {}", select.duration));
        }
        Some(d) => *state.duration.lock().unwrap() = d,
    }

    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/control.html"))
        .finish()
}
