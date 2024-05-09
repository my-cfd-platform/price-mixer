use crate::{app::AppContext, models::PriceMixerBidAskModel};
use cfd_engine_sb_contracts::BidAskSbModel;
use my_nosql_contracts::{MarkupProfileNoSqlEntity, TradingInstrumentNoSqlEntity};
use service_sdk::my_logger::LogEventCtx;
use service_sdk::rust_extensions::events_loop::EventsLoopTick;
use std::sync::Arc;

use super::{map_bid_ask_to_sb_model, map_bid_ask_to_sb_model_with_markup};

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
    async fn started(&self) {
        println!("PublishPricesLoop started");
    }

    async fn finished(&self) {
        println!("PublishPricesLoop finished");
    }

    async fn tick(&self, _: ()) {
        if let Some(messages_to_publish) = self.get_messages_to_publish().await {
            let mark_ups = self
                .app
                .markups
                .get_entity(
                    MarkupProfileNoSqlEntity::generate_partition_key(),
                    MarkupProfileNoSqlEntity::GLOBAL_PROFILE_ID,
                )
                .await;

            let sb_models: Vec<BidAskSbModel> = match mark_ups {
                Some(profile) => {
                    if profile.disabled {
                        messages_to_publish
                            .into_iter()
                            .map(|message| {
                                return map_bid_ask_to_sb_model(message);
                            })
                            .collect()
                    } else {
                        let mut result = Vec::with_capacity(messages_to_publish.len());

                        for message in messages_to_publish {
                            if message.id == "EURUSD" {
                                println!("------");
                                println!("EURUSD GLOBAL profile. {:?}", profile);
                            }

                            if let Some(profile) = profile.instruments.get(&message.id) {
                                let instrument = self
                                    .app
                                    .instrument_reader
                                    .get_entity(
                                        TradingInstrumentNoSqlEntity::generate_partition_key(),
                                        &message.id,
                                    )
                                    .await;

                                if let Some(instrument) = instrument {
                                    if message.id == "EURUSD" {
                                        println!("EURUSD after GLOBAL profile. {:?}", message);
                                    }

                                    let model = map_bid_ask_to_sb_model_with_markup(
                                        message,
                                        profile.markup_bid,
                                        profile.markup_ask,
                                        instrument.digits,
                                    );

                                    if model.id == "EURUSD" {
                                        println!("EURUSD after global apply. {:?}", model);
                                    }
                                    result.push(model);
                                }
                            } else {
                                result.push(map_bid_ask_to_sb_model(message));
                            }
                        }

                        result
                    }
                }
                None => messages_to_publish
                    .into_iter()
                    .map(|message| {
                        return map_bid_ask_to_sb_model(message);
                    })
                    .collect(),
            };

            let result = self
                .app
                .bid_ask_publisher
                .publish_messages(sb_models.iter().map(|itm| (itm, None)))
                .await;
            if let Err(err) = result {
                service_sdk::my_logger::LOGGER.write_error(
                    "PublishingToSB".to_string(),
                    format!("Err: {:?}", err),
                    LogEventCtx::new(),
                );
            }
        }
    }
}
