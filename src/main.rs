mod server;
mod handler;
mod infrastructure;
mod repositories;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run_server().await
}
