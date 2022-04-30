use actix_web::{get, web, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
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
