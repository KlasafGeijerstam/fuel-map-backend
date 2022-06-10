use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use log::info;
use pretty_env_logger;
use sqlx::PgPool;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tokio;

mod endpoint;
mod infra;
mod repo;
mod service;

use infra::postgres::PostgresDatabaseRepository;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Arguments {
    /// Address to listen on.
    #[clap(short, long, default_value = "0.0.0.0")]
    bind_address: String,

    /// Port to listen for incoming connections on.
    #[clap(short, long, default_value_t = 8080, env("PORT"))]
    port: u16,

    /// Database connection string
    #[clap(long, env("DATABASE_URL"))]
    database_url: String,

    /// Comma separated list of hosts allowed to perform CORS requests.
    /// Example "http://localhost:3000, https://google.com"
    #[clap(long, env("CORS_HOSTS"))]
    cors_hosts: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Arguments::parse();

    println!("Listening on {}:{}", args.bind_address, args.port);

    let db_pool = Arc::new(infra::postgres::create_pool(&args.database_url).await?);

    HttpServer::new(move || {
        let mut cors = Cors::default().allow_any_header().allow_any_method();
        if let Some(cors_hosts) = args.cors_hosts.clone() {
            for host in cors_hosts.split(",") {
                info!("Adding {host} to CORS whitelist");
                cors = cors.allowed_origin(host.trim());
            }
        }

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(|cfg| configure_sites(db_pool.clone(), cfg))
    })
    .bind((args.bind_address, args.port))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}

fn configure_sites(pg_pool: Arc<PgPool>, cfg: &mut web::ServiceConfig) {
    let site_service = service::SiteServiceImpl {
        site_repo: PostgresDatabaseRepository { pool: pg_pool },
    };
    endpoint::configure(web::Data::new(site_service), cfg);
}
