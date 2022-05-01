use actix_web::{App, HttpServer};

use clap::Parser;
use tokio;

mod endpoints;
mod infrastructure;
mod repo;

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
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    println!("Listening on {}:{}", args.bind_address, args.port);

    HttpServer::new(|| App::new().configure(endpoints::configure))
        .bind((args.bind_address, args.port))?
        .run()
        .await
}
