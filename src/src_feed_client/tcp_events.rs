use std::sync::Arc;

use my_tcp_sockets::{ConnectionEvent, SocketEventCallback};
use prices_tcp_contracts::{BidAskTcpMessage, BidAskTcpSerializer};

use crate::app::AppContext;

pub struct TcpConnectionEvents {
    app_ctx: Arc<AppContext>,
    id: String,
}

impl TcpConnectionEvents {
    pub fn new(app_ctx: Arc<AppContext>, id: String) -> Self {
        Self { app_ctx, id }
    }
}

#[async_trait::async_trait]
impl SocketEventCallback<BidAskTcpMessage, BidAskTcpSerializer> for TcpConnectionEvents {
    async fn handle(
        &self,
        connection_event: ConnectionEvent<BidAskTcpMessage, BidAskTcpSerializer>,
    ) {
        match connection_event {
            ConnectionEvent::Connected(_) => {
                println!("Connected to Feed");
            }
            ConnectionEvent::Disconnected(_) => {
                println!("Disconnected from Feed");
            }
            ConnectionEvent::Payload {
                connection,
                payload,
            } => {
                if payload.is_ping() {
                    connection.send(BidAskTcpMessage::Pong).await;
                    return;
                }

                if let BidAskTcpMessage::BidAsk(bid_ask) = payload {
                    crate::operations::input_filter::process(&self.app_ctx, bid_ask, &self.id)
                        .await;
                }
            }
        }
    }
}
