#![deny(clippy::all)]

use actix_web::{middleware::Logger, App, HttpServer};
use clap::Arg;
use taskchampion_sync_server::storage::SqliteStorage;
use taskchampion_sync_server::Server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let matches = clap::App::new("taskchampion-sync-server")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Server for TaskChampion")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Port on which to serve")
                .default_value("8080")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("data-dir")
                .short("d")
                .long("data-dir")
                .value_name("DIR")
                .help("Directory in which to store data")
                .default_value("/var/lib/taskchampion-sync-server")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let data_dir = matches.value_of("data-dir").unwrap();
    let port = matches.value_of("port").unwrap();

    let server = Server::new(Box::new(SqliteStorage::new(data_dir)?));

    log::warn!("Serving on port {}", port);
    HttpServer::new(move || App::new().wrap(Logger::default()).service(server.service()))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{test, App};
    use taskchampion_sync_server::storage::{InMemoryStorage, Storage};

    #[actix_rt::test]
    async fn test_index_get() {
        let server = Server::new(Box::new(InMemoryStorage::new()));
        let mut app = test::init_service(App::new().service(server.service())).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}