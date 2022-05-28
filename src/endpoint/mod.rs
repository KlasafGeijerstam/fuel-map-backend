use crate::service::SiteService;
use actix_web::{get, http::StatusCode, web, HttpResponse, Responder, ResponseError};
use std::fmt::Display;

pub mod site;

pub fn configure<S: 'static + SiteService>(
    site_service: web::Data<S>,
    cfg: &mut web::ServiceConfig,
) {
    cfg.app_data(site_service);
    cfg.service(index);
    cfg.service(web::scope("/api/v1").service(site::get_service::<S>()));
}

#[get("/{id}/{name}/")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_index_ok() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/1/test/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[derive(Debug)]
pub struct NotFoundError();

impl NotFoundError {
    pub fn new() -> Self {
        Self()
    }
}

impl Display for NotFoundError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl ResponseError for NotFoundError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
}
