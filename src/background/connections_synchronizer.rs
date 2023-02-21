use crate::{
    app::{AppContext, BridgeConnection},
    bridge_config::BridgeConfig,
    src_feed_client::TcpConnectionEvents,
};
use my_tcp_sockets::TcpClient;
use prices_tcp_contracts::BidAskTcpSerializer;
use rust_extensions::MyTimerTick;
use std::sync::Arc;

pub struct ConnectionsSynchronizerTimer {
    app: Arc<AppContext>,
}

impl ConnectionsSynchronizerTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for ConnectionsSynchronizerTimer {
    async fn tick(&self) {
        let bridges_config: Vec<Arc<BridgeConfig>> = self
            .app
            .settings
            .bridges_config
            .split("|")
            .map(|x| Arc::new(BridgeConfig::from(x)))
            .collect();
        let mut bridge_connections = self.app.bridge_connections.lock().await;

        for bridge_settings in bridges_config {
            if bridge_connections.contains_key(&bridge_settings.name) {
                continue;
            }
            print!("Bringing up {} bridge", &bridge_settings.name);

            let bridge_connection = BridgeConnection {
                host_port: bridge_settings.host_port.clone(),
                tcp_client: TcpClient::new(
                    format!("TcpClient: {}", &bridge_settings.name),
                    bridge_settings.clone(),
                ),
            };

            bridge_connection
                .tcp_client
                .start(
                    Arc::new(|| -> BidAskTcpSerializer { BidAskTcpSerializer::new() }),
                    Arc::new(TcpConnectionEvents::new(
                        self.app.clone(),
                        bridge_settings.name.clone(),
                    )),
                    self.app.logger.clone(),
                )
                .await;

            bridge_connections.insert(bridge_settings.name.clone(), bridge_connection);
        }
    }
}
