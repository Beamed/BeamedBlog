pub mod login_controller;
use actix_web::{HttpRequest, HttpResponse, http, Error};

pub fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("This is the API for Beamed's Blog. Since this is a RESTful API, you probably don't want to be here."))
}