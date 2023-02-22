use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;

use crate::app::{AppContext, APP_NAME, APP_VERSION};

pub fn setup_server(app: Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8080)));

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
    )));

    http_server.start(app.app_states.clone(), app.logger.clone());
}
