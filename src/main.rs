mod prelude{
    pub use crate::models;
    pub use crate::handlers;
}

mod routes;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = handlers::database::pool::create().await.expect("Database connection error");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(models::application::AppState {
                pg_pool: pool.clone(),
            }))
            .service(routes::get_health)
            .service(routes::post_auth_register)
            .service(routes::post_auth_login)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}