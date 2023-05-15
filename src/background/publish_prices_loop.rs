use crate::{app::AppContext, models::PriceMixerBidAskModel};
use cfd_engine_sb_contracts::BidAskSbModel;
use rust_extensions::events_loop::EventsLoopTick;
use std::sync::Arc;

use super::map_bid_ask_to_sb_model;

pub struct PublishPricesLoop {
    pub app: Arc<AppContext>,
}

impl PublishPricesLoop {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }

    async fn get_messages_to_publish(&self) -> Option<Vec<PriceMixerBidAskModel>> {
        let mut write_access = self.app.bid_ask_to_publish.lock().await;
        if write_access.len() == 0 {
            return None;
        }

        let mut result = Vec::new();
        std::mem::swap(&mut result, &mut *write_access);

        Some(result)
    }
}

#[async_trait::async_trait]
impl EventsLoopTick<()> for PublishPricesLoop {
    async fn tick(&self, _: ()) {
        if let Some(messages_to_publish) = self.get_messages_to_publish().await {
            let sb_models: Vec<BidAskSbModel> = messages_to_publish
                .iter()
                .map(|message| {
                    return map_bid_ask_to_sb_model(message);
                })
                .collect();
            let result = self.app.bidask_publisher.publish_messages(&sb_models).await;
            if let Err(err) = result {
                self.app.logger.write_log(
                    my_logger::LogLevel::Error,
                    "PublishingToSB".to_string(),
                    format!("Err: {:?}", err),
                    None,
                );
            }
        }
    }
}
