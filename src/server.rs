//! Spin up a HTTPServer

use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let app = move || {
        App::new()
            .wrap(Cors::new().finish())
            .wrap(Logger::default())
            .configure(add_pool)
            .configure(routes)
    };
    // wraps on new app are ordered from most internal to most external
    let mut server = HttpServer::new(app);

    // Can listen to file dir for changes and auto reload of server
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}
