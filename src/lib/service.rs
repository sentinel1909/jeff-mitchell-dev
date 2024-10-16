// src/lib/service.rs

// dependencies
use axum::{extract::Request, Router, ServiceExt};
use shuttle_runtime::Error;

// struct type to represent the server service
pub struct JeffMitchellDevService {
    pub app_router: Router,
}

// implement the Shuttle Service trait on the JeffMitchellDevService type
#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for JeffMitchellDevService {
    async fn bind(self, addr: std::net::SocketAddr) -> Result<(), Error> {
        let router = self.app_router;

        axum::serve(
            tokio::net::TcpListener::bind(addr).await?,
            ServiceExt::<Request>::into_make_service(router),
        )
        .await?;

        Ok(())
    }
}
