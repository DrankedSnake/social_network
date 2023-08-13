mod users;

use actix_web::{web::{self}, App, HttpServer};
use users::get_users;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
        .route("/users", web::get().to(get_users))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}