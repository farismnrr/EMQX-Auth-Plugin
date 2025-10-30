use actix_web::{App, HttpServer, middleware, web};
use std::sync::Arc;

use crate::infrastructure::rocksdb::init_rocksdb;
use crate::handler::user_handler::{create_user_handler, AppState};
use crate::services::user_create_service::UserService;
use crate::repositories::user_repository::UserRepository;

pub async fn run_server() -> std::io::Result<()> {
    let db = init_rocksdb("./rocksdb-data/iotnet").expect("Failed to init RocksDB");

    // construct repository (owns the DB handle) and then the service that uses the repository
    let repo = Arc::new(UserRepository::new(Arc::clone(&db)));
    let user_service = UserService::new(Arc::clone(&repo));
    let state = web::Data::new(AppState { service: Arc::new(user_service) });
    // clone specifically for the server closure so the original `state` stays
    // available here for explicit drop/cleanup after the server stops
    let state_for_server = state.clone();

    println!("🚀 Actix server running on http://0.0.0.0:5000");

    let server_result = HttpServer::new(move || {
        App::new()
            .app_data(state_for_server.clone())
            .wrap(middleware::Compress::default())
            // Add API routes here
            .route("/users", web::post().to(create_user_handler))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await;

    // After the server stops, drop local references to repository/service so
    // the last Arc clones to the RocksDB instance are released, then attempt
    // to close the DB cleanly.
    drop(state);
    drop(repo);
    crate::infrastructure::rocksdb::close_rocksdb(db);

    server_result
}
