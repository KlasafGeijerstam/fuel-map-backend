use crate::repo::site::{Site, SiteId, SiteRepository};
use anyhow::Result;
use async_trait::async_trait;
use sqlx;
use sqlx::PgPool;
use std::sync::Arc;

pub struct PostgresDatabaseRepository {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl SiteRepository for PostgresDatabaseRepository {
    async fn all(&self) -> Result<Vec<Site>> {
        sqlx::query_as!(Site, "SELECT * FROM site ")
            .fetch_all(&*self.pool)
            .await
            .map_err(anyhow::Error::from)
    }

    async fn delete(&self, id: SiteId) -> Result<Option<Site>> {
        sqlx::query_as!(Site, "DELETE FROM site WHERE id = $1 RETURNING *", id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(anyhow::Error::from)
    }

    async fn create(&self, site: Site) -> Result<Site> {
        sqlx::query_as!(
            Site,
            "
            INSERT INTO site(address, lat, lng)
                VALUES($1, $2, $3)
            RETURNING *",
            site.address,
            site.lat,
            site.lng
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    async fn update(&self, id: SiteId, site: Site) -> Result<Site> {
        sqlx::query_as!(
            Site,
            "
            UPDATE site SET address = $1, lat = $2, lng = $3
            WHERE id = $4
            RETURNING *",
            site.address,
            site.lat,
            site.lng,
            id,
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(anyhow::Error::from)
    }
}

pub async fn create_pool(pg_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(pg_url).await
}

impl PostgresDatabaseRepository {
    pub async fn connect(pg_url: &str) -> Result<Self, sqlx::Error> {
        let pool = create_pool(pg_url).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresDatabaseRepository;
    use super::Site;
    use super::SiteRepository;
    use sqlx::migrate::Migrator;
    use std::path::Path;
    use testcontainers::{clients, images::postgres, Container};

    fn get_docker() -> clients::Cli {
        clients::Cli::docker()
    }

    async fn setup_db(
        docker: &clients::Cli,
    ) -> (
        PostgresDatabaseRepository,
        Container<'_, postgres::Postgres>,
    ) {
        let postgres = docker.run(postgres::Postgres::default());
        let pg_url = format!(
            "postgres://postgres@172.17.0.1:{}/postgres",
            postgres.get_host_port(5432)
        );
        let pool = PostgresDatabaseRepository::connect(&pg_url).await.unwrap();
        let migrator = Migrator::new(Path::new(&format!(
            "{}/migrations",
            env!("CARGO_MANIFEST_DIR")
        )))
        .await
        .unwrap();
        migrator.run(&*pool.pool).await.unwrap();

        (pool, postgres)
    }

    #[actix_web::test]
    async fn test_create_sites() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;

        let site = repo
            .create(Site {
                id: 0,
                address: "Street 1".into(),
                lat: "59".into(),
                lng: "58".into(),
            })
            .await
            .unwrap();

        assert_eq!("Street 1", site.address);
        assert_eq!("59", site.lat);
        assert_eq!("58", site.lng);
    }

    #[actix_web::test]
    async fn test_get_sites() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        for i in 0..3 {
            repo.create(Site {
                id: 0,
                address: format!("Street {i}"),
                lat: "50".into(),
                lng: "51".into(),
            })
            .await
            .unwrap();
        }

        let sites = repo.all().await.unwrap();
        assert_eq!(3, sites.len());
    }

    #[actix_web::test]
    async fn test_delete_site() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let site = repo
            .create(Site {
                id: 0,
                address: format!("Street 1"),
                lat: "50".into(),
                lng: "51".into(),
            })
            .await
            .unwrap();

        let deleted_site = repo.delete(site.id).await.unwrap().unwrap();
        assert_eq!(site.address, deleted_site.address);
        let site_count = repo.all().await.unwrap().len();
        assert_eq!(0, site_count);
    }

    #[actix_web::test]
    async fn test_delete_non_existing_site_returns_none() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let site = repo
            .create(Site {
                id: 0,
                address: format!("Street 1"),
                lat: "50".into(),
                lng: "51".into(),
            })
            .await
            .unwrap();

        let deleted_site = repo.delete(site.id + 1).await.unwrap();
        assert!(deleted_site.is_none());
        let site_count = repo.all().await.unwrap().len();
        assert_eq!(1, site_count);
    }

    #[actix_web::test]
    async fn test_update_site() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let site = repo
            .create(Site {
                id: 0,
                address: format!("Street 1"),
                lat: "50".into(),
                lng: "51".into(),
            })
            .await
            .unwrap();

        let updated_site = repo
            .update(
                site.id,
                Site {
                    address: "New Street".into(),
                    ..site
                },
            )
            .await
            .unwrap();
        assert_eq!("New Street", updated_site.address);
        assert_eq!("50", updated_site.lat);
        assert_eq!("51", updated_site.lng);
    }

    #[actix_web::test]
    async fn test_update_non_existing_site_fails() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;

        let result = repo
            .update(
                0,
                Site {
                    id: 0,
                    address: "New Street".into(),
                    lat: "50".into(),
                    lng: "51".into(),
                },
            )
            .await;
        assert_eq!(true, result.is_err());
    }
}
