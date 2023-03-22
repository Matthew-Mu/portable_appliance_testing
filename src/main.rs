mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::appliance::{get_115, get_240, get_out_of_date, get_previous_appliances, submit_private};
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config_aws = aws_config::load_from_env().await;
    HttpServer::new(move || {
        let ddb_repo: DDBRepository =
            DDBRepository::init(String::from("appliance"), config_aws.clone());
        let ddb_data = Data::new(ddb_repo);
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(submit_private)
            .service(get_previous_appliances)
            .service(get_240)
            .service(get_115)
            .service(get_out_of_date)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
