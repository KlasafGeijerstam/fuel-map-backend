pub use crate::repo::site::{NewSite, Site, SiteId, SiteRepository};
use actix_web::{error::ErrorInternalServerError, Result};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SiteService {
    async fn get_sites(&self) -> Result<Vec<Site>>;
    async fn create_site(&self, new_site: NewSite) -> Result<Site>;
    async fn update_site(&self, site_id: SiteId, site: NewSite) -> Result<Option<Site>>;
    async fn delete_site(&self, site_id: SiteId) -> Result<Option<Site>>;
}

pub struct SiteServiceImpl<S: SiteRepository> {
    pub site_repo: S,
}

#[async_trait]
impl<S> SiteService for SiteServiceImpl<S>
where
    S: SiteRepository + Send + Sync,
{
    async fn get_sites(&self) -> Result<Vec<Site>> {
        self.site_repo
            .all()
            .await
            .map_err(|e| ErrorInternalServerError(e))
    }

    async fn create_site(&self, new_site: NewSite) -> Result<Site> {
        self.site_repo
            .create(new_site)
            .await
            .map_err(|e| ErrorInternalServerError(e))
    }

    async fn update_site(&self, site_id: SiteId, site: NewSite) -> Result<Option<Site>> {
        self.site_repo
            .update(site_id, site)
            .await
            .map_err(|e| ErrorInternalServerError(e))
    }

    async fn delete_site(&self, site_id: SiteId) -> Result<Option<Site>> {
        self.site_repo
            .delete(site_id)
            .await
            .map_err(|e| ErrorInternalServerError(e))
    }
}

#[cfg(test)]
mod tests {
    use super::{SiteService, SiteServiceImpl};
    use crate::repo::site::{factory::*, MockSiteRepository};
    use factori::create;

    #[actix_web::test]
    #[test]
    async fn test_get_sites() {
        let mut site_repo = MockSiteRepository::new();
        site_repo
            .expect_all()
            .returning(|| Ok(vec![create!(Site, id: 0), create!(Site, id: 1)]));

        let service = SiteServiceImpl { site_repo };

        let sites = service.get_sites().await.unwrap();
        assert_eq!(2, sites.len());
        assert_eq!(0, sites[0].id);
        assert_eq!(1, sites[1].id);
    }
}
