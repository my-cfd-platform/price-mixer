use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;

use crate::app::AppContext;

pub fn setup_server(app: Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8080)));

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        app.app_name.clone(),
        app.app_version.clone(),
    )));

    http_server.start(app.app_states.clone(), app.logger.clone());
}
