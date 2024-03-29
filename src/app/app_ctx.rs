use super::BridgeConnection;
use crate::{
    models::PriceMixerBidAskModel,
    nosql::{DefaultValuesEntity, InstrumentSourcesEntity},
    settings_model::SettingsReader,
};
use cfd_engine_sb_contracts::BidAskSbModel;
use my_nosql_contracts::{ProductSettings, TradingInstrumentNoSqlEntity};
use service_sdk::{
    my_no_sql_sdk::reader::MyNoSqlDataReaderTcp, rust_extensions::events_loop::EventsLoop,
};
use service_sdk::{my_service_bus::abstractions::publisher::MyServiceBusPublisher, ServiceContext};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct AppContext {
    pub bridge_connections: Mutex<HashMap<String, BridgeConnection>>,
    pub bid_ask_to_publish: Mutex<Vec<PriceMixerBidAskModel>>,
    pub publish_prices_loop: EventsLoop<()>,
    pub instrument_sources_reader: Arc<MyNoSqlDataReaderTcp<InstrumentSourcesEntity>>,
    pub instrument_reader: Arc<MyNoSqlDataReaderTcp<TradingInstrumentNoSqlEntity>>,
    pub defaults_reader: Arc<MyNoSqlDataReaderTcp<DefaultValuesEntity>>,
    pub price_bridges_settings: Arc<MyNoSqlDataReaderTcp<ProductSettings>>,
    pub bid_ask_publisher: MyServiceBusPublisher<BidAskSbModel>,
    pub settings: Arc<SettingsReader>,
}

impl AppContext {
    pub async fn new(settings: Arc<SettingsReader>, sc: &ServiceContext) -> Self {
        Self {
            bridge_connections: Mutex::new(HashMap::new()),
            publish_prices_loop: EventsLoop::new("Output Mixer".to_string()),
            bid_ask_to_publish: Mutex::new(Vec::new()),
            instrument_sources_reader: sc.get_ns_reader().await,
            instrument_reader: sc.get_ns_reader().await,
            defaults_reader: sc.get_ns_reader().await,
            price_bridges_settings: sc.get_ns_reader().await,
            bid_ask_publisher: sc.get_sb_publisher(false).await,
            settings,
        }
    }
}
