use crate::app::{AppContext, ToPublishItem};
use cfd_engine_sb_contracts::BidAskSbModel;
use my_nosql_contracts::MarkupProfileNoSqlEntity;
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
        let messages_to_publish = self.app.bid_ask_to_publish.get_messages_to_publish().await;

        if messages_to_publish.is_none() {
            return;
        }

        let messages_to_publish = messages_to_publish.unwrap();

        let mark_ups = self
            .app
            .markups
            .get_entity(
                MarkupProfileNoSqlEntity::generate_partition_key(),
                MarkupProfileNoSqlEntity::GLOBAL_PROFILE_ID,
            )
            .await;

        let sb_models: Vec<BidAskSbModel> = match mark_ups {
            Some(markup_profile) => {
                if markup_profile.disabled {
                    messages_to_publish
                        .into_iter()
                        .map(|message| {
                            return map_bid_ask_to_sb_model(message);
                        })
                        .collect()
                } else {
                    compile_with_markup_profile(messages_to_publish, markup_profile.as_ref())
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

fn compile_with_markup_profile(
    messages_to_publish: Vec<ToPublishItem>,
    markup_profile: &MarkupProfileNoSqlEntity,
) -> Vec<BidAskSbModel> {
    let mut result = Vec::with_capacity(messages_to_publish.len());

    for message in messages_to_publish {
        if message.bid_ask.instrument_id == "EURUSD" {
            println!("------");
            println!("EURUSD GLOBAL profile. {:?}", markup_profile);
        }

        if let Some(instrument_markup) = markup_profile
            .instruments
            .get(&message.bid_ask.instrument_id)
        {
            if message.bid_ask.instrument_id == "EURUSD" {
                println!("EURUSD after GLOBAL profile. {:?}", message.bid_ask);
            }

            let model = map_bid_ask_to_sb_model_with_markup(
                message,
                instrument_markup.markup_bid,
                instrument_markup.markup_ask,
            );

            if model.id == "EURUSD" {
                let spread = ((model.ask - model.bid) * 100000.0) as i64;

                println!("EURUSD after global apply. {:?}. Spread: {}", model, spread);
            }
            result.push(model);
        } else {
            result.push(map_bid_ask_to_sb_model(message));
        }
    }

    result
}
