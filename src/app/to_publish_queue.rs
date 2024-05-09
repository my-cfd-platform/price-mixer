use std::sync::Arc;

use my_nosql_contracts::TradingInstrumentNoSqlEntity;
use prices_tcp_contracts::BidAskDataTcpModel;
use tokio::sync::Mutex;

pub struct ToPublishItem {
    pub instrument: Arc<TradingInstrumentNoSqlEntity>,
    pub bid_ask: BidAskDataTcpModel,
}

pub struct ToPublishQueue {
    pub inner: Mutex<Vec<ToPublishItem>>,
}

impl ToPublishQueue {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Vec::new()),
        }
    }

    pub async fn publish(
        &self,
        instrument: Arc<TradingInstrumentNoSqlEntity>,
        bid_ask: BidAskDataTcpModel,
    ) {
        let mut write_access = self.inner.lock().await;
        write_access.push(ToPublishItem {
            instrument,
            bid_ask,
        });
    }

    pub async fn get_messages_to_publish(&self) -> Option<Vec<ToPublishItem>> {
        let mut write_access = self.inner.lock().await;
        if write_access.len() == 0 {
            return None;
        }

        let mut result = Vec::new();
        std::mem::swap(&mut result, &mut *write_access);

        Some(result)
    }
}
