use crate::service::SiteService;
use actix_web::web::Json;
use actix_web::{error, web, Responder, Result};

pub fn configure<T: 'static + SiteService>(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").route("/sites", web::get().to(get_sites::<T>)));
}

async fn get_sites<T: SiteService>(site_repo: web::Data<T>) -> Result<impl Responder> {
    println!("OID");
    let sites = site_repo
        .get_sites()
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(Json(sites))
}
