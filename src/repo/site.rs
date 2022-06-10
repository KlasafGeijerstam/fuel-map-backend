use anyhow::Result;
use async_trait::async_trait;
pub use model::{NewSite, Site, SiteId};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait SiteRepository {
    async fn get(&self, id: SiteId) -> Result<Option<Site>>;
    async fn all(&self) -> Result<Vec<Site>>;
    async fn delete(&self, id: SiteId) -> Result<Option<Site>>;
    async fn create(&self, site: NewSite) -> Result<Site>;
    async fn update(&self, id: SiteId, site: NewSite) -> Result<Option<Site>>;
}

pub mod model {
    use serde::{Deserialize, Serialize};
    use sqlx;

    pub type SiteId = i32;

    #[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
    pub struct Site {
        pub id: SiteId,
        pub address: String,
        pub lat: String,
        pub lng: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct NewSite {
        pub address: String,
        pub lat: String,
        pub lng: String,
    }
}

#[cfg(test)]
pub mod factory {
    use super::model::{NewSite, Site};
    use factori::factori;

    factori!(Site, {
        default {
            id = 0,
            address = "Address".into(),
            lat = "59.0".into(),
            lng = "58.0".into(),
        }
    });

    factori!(NewSite, {
        default {
            address = "Address".into(),
            lat = "59.0".into(),
            lng = "58.0".into(),
        }
    });
}
