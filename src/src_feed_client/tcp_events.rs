use std::sync::Arc;

use my_tcp_sockets::{tcp_connection::TcpSocketConnection, SocketEventCallback};
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
impl SocketEventCallback<BidAskTcpMessage, BidAskTcpSerializer, ()> for TcpConnectionEvents {
    async fn connected(
        &self,
        _connection: Arc<TcpSocketConnection<BidAskTcpMessage, BidAskTcpSerializer, ()>>,
    ) {
        println!("Connected to Feed {}", self.id);
    }

    async fn disconnected(
        &self,
        _connection: Arc<TcpSocketConnection<BidAskTcpMessage, BidAskTcpSerializer, ()>>,
    ) {
        println!("Disconnected from Feed {}", self.id);
    }

    async fn payload(
        &self,
        connection: &Arc<TcpSocketConnection<BidAskTcpMessage, BidAskTcpSerializer, ()>>,
        contract: BidAskTcpMessage,
    ) {
        if contract.is_ping() {
            connection.send(&BidAskTcpMessage::Pong).await;
            return;
        }

        if let BidAskTcpMessage::BidAsk(bid_ask) = contract {
            crate::operations::input_filter::process(&self.app_ctx, bid_ask, &self.id).await;
        }
    }
}
