use crate::State;
use crate::db::{get_title, list_titles};
use crate::state::Duration;
use actix_web::web::{Data, Redirect, ServiceConfig};
use actix_web::{HttpResponse, Responder, get};
use sailfish::TemplateSimple;

pub fn frontend(cfg: &mut ServiceConfig) {
    cfg.service(index)
        .service(control_css)
        .service(control_html)
        .service(control_js)
        .service(view_html)
        .service(view_css)
        .service(logo);
}

#[get("/")]
async fn index(_: Data<State>) -> impl Responder {
    Redirect::to("/control.html").permanent()
}

#[get("/control.css")]
pub async fn control_css() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(include_str!("../static/control.css"))
}

#[get("/control.js")]
pub async fn control_js() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(include_str!("../static/control.js"))
}

#[get("/logo.png")]
pub async fn logo() -> impl Responder {
    let logo = match std::fs::read("logo.png") {
        Ok(logo) => logo,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    HttpResponse::Ok().content_type("image/png").body(logo)
}

#[derive(TemplateSimple)]
#[template(path = "control.html")]
struct ControlHtml {
    current_name: String,
    current_label: String,
    duration: Duration,
    titles: Vec<(i64, String, String)>,
}

#[get("/control.html")]
pub async fn control_html(state: Data<State>) -> impl Responder {
    let records = match list_titles() {
        Ok(res) => res,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let selected = *state.selected.lock().unwrap();
    let (mut current_name, mut current_label) = (String::new(), String::new());
    let mut titles = Vec::new();
    for (id, name, label) in records {
        if let Some(selected) = selected
            && selected == id
        {
            current_name = name.clone();
            current_label = label.clone();
        }
        titles.push((id, name, label));
    }

    let duration = *state.duration.lock().unwrap();

    let html = ControlHtml {
        current_name,
        current_label,
        duration,
        titles,
    }
    .render_once()
    .unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[derive(TemplateSimple)]
#[template(path = "view.html")]
struct ViewHtml {
    name: String,
    label: String,
}

#[get("/view.html")]
pub async fn view_html(state: Data<State>) -> impl Responder {
    let selected = *state.selected.lock().unwrap();
    if selected.is_none() {
        return HttpResponse::BadRequest().body("No title selected!");
    }
    let selected = selected.unwrap();

    let (name, label) = match get_title(selected) {
        Ok(res) => {
            if let Some(res) = res {
                res
            } else {
                return HttpResponse::BadRequest().body(format!("No title with id {}", selected));
            }
        }
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let html = ViewHtml { name, label }.render_once().unwrap();

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[derive(TemplateSimple)]
#[template(path = "view.css")]
struct ViewCss {
    duration: u8,
    obj: f32,
    text_delay: f32,
    text: f32,
    fade: f32,
    manual: bool,
}

#[get("/view.css")]
pub async fn view_css(state: Data<State>) -> impl Responder {
    let duration = *state.duration.lock().unwrap();
    let manual = duration.is_manual();
    let duration = duration.get_duration();

    let (obj, text_delay, text, mut fade) = calc_percentages(duration);

    if manual {
        fade = 100.;
    }

    let css = ViewCss {
        duration,
        obj,
        text_delay,
        text,
        fade,
        manual,
    }
    .render_once()
    .unwrap();

    HttpResponse::Ok().content_type("text/css").body(css)
}

fn calc_percentages(duration: u8) -> (f32, f32, f32, f32) {
    const OBJ_INTRO: f32 = 0.95;
    const TEXT_DELAY: f32 = 0.9;
    const TEXT_INTRO: f32 = 0.55;
    const FADE: f32 = 0.5;

    let duration = duration as f32;
    let obj = OBJ_INTRO / duration;
    let text_delay = TEXT_DELAY / duration;
    let text = text_delay + (TEXT_INTRO / duration);
    let fade = 1. - (FADE / duration);

    (obj * 100., text_delay * 100., text * 100., fade * 100.)
}
