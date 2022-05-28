use crate::endpoint::NotFoundError;
use crate::service::{NewSite, SiteId, SiteService};
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{web, Responder, Result};

pub fn get_service<T: 'static + SiteService>() -> impl HttpServiceFactory + 'static {
    web::scope("/sites")
        .route("", web::get().to(get_sites::<T>))
        .route("", web::post().to(create_site::<T>))
        .service(
            web::scope("/{site_id}")
                .route("", web::put().to(update_site::<T>))
                .route("", web::delete().to(delete_site::<T>)),
        )
}

async fn get_sites<T: SiteService>(site_repo: web::Data<T>) -> Result<impl Responder> {
    let sites = site_repo.get_sites().await?;

    Ok(Json(sites))
}

async fn create_site<T: SiteService>(
    site: web::Json<NewSite>,
    site_repo: web::Data<T>,
) -> Result<impl Responder> {
    let site = site_repo.create_site(site.into_inner()).await?;

    Ok(Json(site))
}

async fn update_site<T: SiteService>(
    site_id: web::Path<SiteId>,
    site: web::Json<NewSite>,
    site_repo: web::Data<T>,
) -> Result<impl Responder> {
    let site = site_repo
        .update_site(*site_id, site.into_inner())
        .await?
        .ok_or(NotFoundError::new())?;
    Ok(Json(site))
}

async fn delete_site<T: SiteService>(
    site_id: web::Path<SiteId>,
    site_repo: web::Data<T>,
) -> Result<impl Responder> {
    let site = site_repo
        .delete_site(*site_id)
        .await?
        .ok_or(NotFoundError::new())?;
    Ok(Json(site))
}

#[cfg(test)]
mod tests {
    use super::get_service;
    use crate::repo::site::factory::*;
    use crate::service::MockSiteService;
    use actix_web::{http::StatusCode, test, web::Data, App};
    use factori::create;

    #[actix_web::test]
    async fn test_get_empty_sites() {
        let mut site_service = MockSiteService::new();
        site_service.expect_get_sites().returning(|| Ok(vec![]));

        let app = test::init_service(
            App::new()
                .app_data(Data::new(site_service))
                .service(get_service::<MockSiteService>()),
        )
        .await;
        let req = test::TestRequest::get().uri("/sites").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_non_existing_site_returns_404() {
        let mut site_service = MockSiteService::new();
        site_service.expect_update_site().returning(|_, _| Ok(None));

        let app = test::init_service(
            App::new()
                .app_data(Data::new(site_service))
                .service(get_service::<MockSiteService>()),
        )
        .await;
        let site = create!(NewSite);
        let req = test::TestRequest::put()
            .uri("/sites/3")
            .set_json(site)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_delete_non_existing_site_returns_404() {
        let mut site_service = MockSiteService::new();
        site_service.expect_delete_site().returning(|_| Ok(None));

        let app = test::init_service(
            App::new()
                .app_data(Data::new(site_service))
                .service(get_service::<MockSiteService>()),
        )
        .await;
        let req = test::TestRequest::delete().uri("/sites/3").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
