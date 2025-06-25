use actix_web::{App, HttpResponse, HttpServer, web};
use cobblepot::{account, apply, balance, infrastructure, recurring_transaction, report};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_pool = infrastructure::database::database_pool().unwrap();

    let server = HttpServer::new(move || {
        App::new()
            // ~~~ API Spec
            // ~~~ App data
            .app_data(web::Data::new(database_pool.clone()))
            // ~~~ Global Middleware
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            // ~~~ Routes
            .route("/health_check", web::get().to(|| HttpResponse::Ok()))
            .service(account::service::account_scope())
            .service(balance::service::balance_scope())
            .service(report::service::report_scope())
            .service(recurring_transaction::service::recurring_transaction_scope())
            .service(apply::service::apply_scope())
    })
    .bind(("127.0.0.1", 8080))
    .expect("Failed to bind to address");

    server.run().await
}
