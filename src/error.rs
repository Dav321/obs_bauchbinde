use actix_web::HttpResponse;
use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::ServiceResponse;
use actix_web::http::header::{CONTENT_TYPE, HeaderValue};
use actix_web::middleware::ErrorHandlerResponse;
use sailfish::TemplateSimple;
use serde::Deserialize;

#[derive(TemplateSimple, Deserialize, Clone)]
#[template(path = "error.html")]
struct ErrorHtml {
    code: u16,
    name: String,
    message: String,
}

pub fn error_html(code: u16, name: String, message: String) -> String {
    let error = ErrorHtml {
        code,
        name,
        message,
    };
    error.clone().render_once().unwrap()
}

pub fn error_middleware(
    res: ServiceResponse<BoxBody>,
) -> actix_web::Result<ErrorHandlerResponse<BoxBody>> {
    if res.request().path() == "/view.html" {
        let res = ServiceResponse::new(res.request().clone(), HttpResponse::Ok().finish());
        return Ok(ErrorHandlerResponse::Response(res.map_into_left_body()));
    }

    let res = res.map_body(|h, b| {
        h.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
        let code = h.status.as_u16();
        let name = h.status.canonical_reason().unwrap_or("").to_string();
        let body = match b.try_into_bytes() {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(_) => "Unknown Error".to_string(),
        };
        BoxBody::new(error_html(code, name, body))
    });

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
