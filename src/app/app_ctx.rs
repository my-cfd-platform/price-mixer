use super::BridgeConnection;
use crate::{
    settings_model::SettingsModel, DefaultValuesEntity,
    InstrumentSourcesEntity, models::PriceMixerBidAskModel,
};
use cfd_engine_sb_contracts::BidAskSbModel;
use my_logger::MyLogger;
use my_no_sql_tcp_reader::MyNoSqlDataReader;
use my_nosql_contracts::TradingInstrumentNoSqlEntity;
use my_service_bus_abstractions::publisher::MyServiceBusPublisher;
use rust_extensions::{events_loop::EventsLoop, AppStates};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub bridge_connections: Mutex<HashMap<String, BridgeConnection>>,
    pub bid_ask_to_publish: Mutex<Vec<PriceMixerBidAskModel>>,
    pub publish_prices_loop: EventsLoop<()>,
    pub logger: Arc<MyLogger>,
    pub app_states: Arc<AppStates>,
    pub settings: Arc<SettingsModel>,
    pub instrument_sources_reader: Arc<MyNoSqlDataReader<InstrumentSourcesEntity>>,
    pub instrument_reader: Arc<MyNoSqlDataReader<TradingInstrumentNoSqlEntity>>,
    pub defaults_reader: Arc<MyNoSqlDataReader<DefaultValuesEntity>>,
    pub bidask_publisher: MyServiceBusPublisher<BidAskSbModel>,
}

impl AppContext {
    pub async fn new(
        settings: Arc<SettingsModel>,
        logger: Arc<MyLogger>,
        instrument_sources_reader: Arc<MyNoSqlDataReader<InstrumentSourcesEntity>>,
        instrument_reader: Arc<MyNoSqlDataReader<TradingInstrumentNoSqlEntity>>,
        defaults_reader: Arc<MyNoSqlDataReader<DefaultValuesEntity>>,
        bidask_publisher: MyServiceBusPublisher<BidAskSbModel>,
    ) -> Self {
        Self {
            bridge_connections: Mutex::new(HashMap::new()),
            publish_prices_loop: EventsLoop::new("Output Mixer".to_string()),
            bid_ask_to_publish: Mutex::new(Vec::new()),
            logger,
            app_states: Arc::new(AppStates::create_initialized()),
            settings,
            instrument_reader,
            instrument_sources_reader,
            defaults_reader,
            bidask_publisher,
        }
    }
}
