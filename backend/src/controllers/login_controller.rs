use actix_web::{HttpRequest, HttpResponse, http, Error};

pub fn handle_login(req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("{}"))
}