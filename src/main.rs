mod server;
mod handler;
mod middleware;
mod infrastructure;
mod repositories;
mod services;
mod entities;
mod utils;
mod dtos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run_server().await
}
