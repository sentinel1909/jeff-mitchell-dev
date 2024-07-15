// src/bin/main.rs

// dependencies
use axum::Router;
use jeff_mitchell_dev_lib::config::ServerConfig;
use jeff_mitchell_dev_lib::Application;
use shuttle_runtime::{Error, Service};

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct ServerService {
    server: Router,
}

// implement the Shuttle Service trait for the ServerService type
#[shuttle_runtime::async_trait]
impl Service for ServerService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), Error> {
        let router = self.server;

        axum::serve(tokio::net::TcpListener::bind(addr).await?, router).await?;

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn shuttle_main() -> Result<ServerService, Error> {
    let config = ServerConfig {};

    let Application(server) = Application::build(config)?;

    Ok(ServerService { server })
}
