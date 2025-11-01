use actix_web::{App, HttpServer, middleware, web};
use std::sync::Arc;

use crate::infrastructure::rocksdb::{init_rocksdb, close_rocksdb};
use crate::middleware::api_key::ApiKeyMiddleware;
use crate::middleware::powered_by::PoweredByMiddleware;

use crate::handler::create_user_handler::{create_user_handler, AppState as CreateUserAppState};
use crate::handler::get_user_list_handler::{get_user_list_handler, AppState as GetListAppState};

use crate::services::create_user_service::CreateUserService;
use crate::services::get_user_list_service::GetUserListService;
use crate::repositories::create_user_repository::CreateUserRepository;
use crate::repositories::get_user_list_repository::GetUserListRepository;

pub async fn run_server() -> std::io::Result<()> {
    // init db
    let db = init_rocksdb("./rocksdb-data/iotnet")
        .expect("❌ Failed to initialize RocksDB");

    // di layer: create repository instances expected by services
    let create_repo = Arc::new(CreateUserRepository::new(Arc::clone(&db)));
    let list_repo = Arc::new(GetUserListRepository::new(Arc::clone(&db)));

    let create_user_service = Arc::new(CreateUserService::new(Arc::clone(&create_repo)));
    let get_user_list_service = Arc::new(GetUserListService::new(Arc::clone(&list_repo)));

    // create per-handler states (each handler defines its own AppState type)
    let create_state = web::Data::new(CreateUserAppState { create_user_service });
    let list_state = web::Data::new(GetListAppState { get_user_list_service });

    println!("🚀 Actix server running on http://0.0.0.0:5500");

    // http server
    let server_result = HttpServer::new(move || {
        App::new()
            .app_data(create_state.clone())
            .app_data(list_state.clone())
            .wrap(PoweredByMiddleware)
            .wrap(middleware::Compress::default())
            .wrap(ApiKeyMiddleware)
            .service(
                web::scope("/users")
                    .route("", web::post().to(create_user_handler))

                    // Development only
                    .route("", web::get().to(get_user_list_handler)),
            )
    })
    .bind(("0.0.0.0", 5500))?
    .run()
    .await;

    // cleanup
    drop(create_repo);
    drop(list_repo);
    close_rocksdb(db);

    server_result
}
