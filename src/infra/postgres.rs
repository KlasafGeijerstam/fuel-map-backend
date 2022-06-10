use crate::repo::site::{NewSite, Site, SiteId, SiteRepository};
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
    async fn get(&self, id: SiteId) -> Result<Option<Site>> {
        sqlx::query_as!(Site, "SELECT * FROM site WHERE id = $1", id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(anyhow::Error::from)
    }

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

    async fn create(&self, site: NewSite) -> Result<Site> {
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

    async fn update(&self, id: SiteId, site: NewSite) -> Result<Option<Site>> {
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
        .fetch_optional(&*self.pool)
        .await
        .map_err(anyhow::Error::from)
    }
}

pub async fn create_pool(pg_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(pg_url).await
}

#[cfg(test)]
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
    use super::NewSite;
    use super::PostgresDatabaseRepository;
    use super::SiteRepository;
    use crate::repo::site::factory::*;
    use factori::create;
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

        let new_site = create!(NewSite);

        let site = repo.create(new_site.clone()).await.unwrap();

        assert_eq!(new_site.address, site.address);
        assert_eq!(new_site.lat, site.lat);
        assert_eq!(new_site.lng, site.lng);
    }

    #[actix_web::test]
    async fn test_get_sites() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        for _ in 0..3 {
            repo.create(create!(NewSite)).await.unwrap();
        }

        let sites = repo.all().await.unwrap();
        assert_eq!(3, sites.len());
    }

    #[actix_web::test]
    async fn test_get_site() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let site = repo.create(create!(NewSite)).await.unwrap();

        let got_site = repo.get(site.id).await.unwrap().unwrap();
        assert_eq!(site.id, got_site.id);
        assert_eq!(site.address, got_site.address);
        assert_eq!(site.lat, got_site.lat);
        assert_eq!(site.lng, got_site.lng);
    }

    #[actix_web::test]
    async fn test_get_non_existing_site() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let got_site = repo.get(0).await.unwrap();
        assert!(got_site.is_none());
    }

    #[actix_web::test]
    async fn test_delete_site() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let site = repo.create(create!(NewSite)).await.unwrap();

        let deleted_site = repo.delete(site.id).await.unwrap().unwrap();
        assert_eq!(site.address, deleted_site.address);
        let site_count = repo.all().await.unwrap().len();
        assert_eq!(0, site_count);
    }

    #[actix_web::test]
    async fn test_delete_non_existing_site_returns_none() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let site = repo.create(create!(NewSite)).await.unwrap();

        let deleted_site = repo.delete(site.id + 1).await.unwrap();
        assert!(deleted_site.is_none());
        let site_count = repo.all().await.unwrap().len();
        assert_eq!(1, site_count);
    }

    #[actix_web::test]
    async fn test_update_site() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;
        let site = repo.create(create!(NewSite)).await.unwrap();

        let updated_site = repo
            .update(
                site.id,
                NewSite {
                    address: "New Street".to_owned(),
                    lat: site.lat.clone(),
                    lng: site.lng.clone(),
                },
            )
            .await
            .unwrap();
        assert_eq!(true, updated_site.is_some());
        let updated_site = updated_site.unwrap();
        assert_eq!("New Street", updated_site.address);
        assert_eq!(site.lat, updated_site.lat);
        assert_eq!(site.lng, updated_site.lng);
    }

    #[actix_web::test]
    async fn test_update_non_existing_site_fails() {
        let docker = get_docker();
        let (repo, _c) = setup_db(&docker).await;

        let result = repo.update(0, create!(NewSite)).await;
        assert_eq!(true, result.unwrap().is_none());
    }
}
