use actix_web::{
    App, HttpResponse, HttpServer,
    web::{self, Data},
};
use env_logger;

mod account;
mod apply;
mod balance;
mod infrastructure;
mod recurring_transaction;
mod report;
mod schema;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_pool = infrastructure::database::database_pool().unwrap();

    let server = HttpServer::new(move || {
        App::new()
            // ~~~ API Spec
            // ~~~ App data
            .app_data(Data::new(database_pool.clone()))
            // ~~~ Global Middleware
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            // ~~~ Routes
            .route("/health_check", web::get().to(|| HttpResponse::Ok()))
            .service(
                web::scope("/open")
                    .route("/account", web::post().to(account::service::create_account))
                    .route("/balance", web::post().to(balance::service::insert_new_balance))
                    .route(
                        "/report_balance_sheet",
                        web::post().to(report::service::create_balance_sheet_report),
                    )
                    .route(
                        "/report_deep_dive",
                        web::post().to(report::service::create_deep_dive_account_report),
                    )
                    .route(
                        "/recurring_transaction",
                        web::post()
                            .to(recurring_transaction::service::insert_new_recurring_transaction),
                    ),
            )
            .service(
                web::scope("/update")
                    .route("/account", web::put().to(account::service::update_account))
                    .route("/balance", web::put().to(balance::service::update_balance)),
            )
            .service(
                web::scope("/close")
                    .route("/account", web::delete().to(account::service::close_account))
                    .route(
                        "/recurring_transaction",
                        web::delete()
                            .to(recurring_transaction::service::close_recurring_transaction),
                    ),
            )
        // .service(web::scope("/apply").route(
        //     "/recurring_transaction",
        //     web::post().to(apply::service::insert_applied_recurring_transaction),
        // ))
    })
    .bind(("127.0.0.1", 8080))
    .expect("Failed to bind to address");

    server.run().await
}
