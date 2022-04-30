use crate::repo::database::DatabaseUtilRepo;
use async_trait::async_trait;
use sqlx;
use sqlx::{query_as, PgPool};
use std::sync::Arc;

pub struct PostgresDatabaseRepo {
    pool: Arc<PgPool>,
}

#[async_trait]
impl DatabaseUtilRepo for PostgresDatabaseRepo {
    async fn alive(&self) -> bool {
        let r: Result<(i64,), sqlx::Error> = query_as("SELECT COUNT(*) FROM site")
            .fetch_one(&*self.pool)
            .await;
        r.is_ok()
    }
}

impl PostgresDatabaseRepo {
    pub async fn connect(pg_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(pg_url).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DatabaseUtilRepo;
    use super::PostgresDatabaseRepo;
    use sqlx::migrate::Migrator;
    use std::path::Path;
    use testcontainers::{clients, images::postgres};

    #[actix_web::test]
    async fn test_alive() {
        let docker = clients::Cli::docker();
        let postgres = docker.run(postgres::Postgres::default());
        let pg_url = format!(
            "postgres://postgres@172.17.0.1:{}/postgres",
            postgres.get_host_port(5432)
        );
        let pool = PostgresDatabaseRepo::connect(&pg_url).await.unwrap();
        let migrator = Migrator::new(Path::new(&format!(
            "{}/migrations",
            env!("CARGO_MANIFEST_DIR")
        )))
        .await
        .unwrap();
        migrator.run(&*pool.pool).await.unwrap();

        assert_eq!(true, pool.alive().await);
    }
}
