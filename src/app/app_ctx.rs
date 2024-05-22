use super::{BridgeConnection, ToPublishQueue};
use crate::settings_model::SettingsReader;
use cfd_engine_sb_contracts::BidAskSbModel;
use my_nosql_contracts::*;
use service_sdk::{
    my_no_sql_sdk::reader::MyNoSqlDataReaderTcp,
    rust_extensions::events_loop::EventsLoopMutexWrapped,
};
use service_sdk::{my_service_bus::abstractions::publisher::MyServiceBusPublisher, ServiceContext};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct AppContext {
    pub bridge_connections: Mutex<HashMap<String, BridgeConnection>>,
    pub bid_ask_to_publish: ToPublishQueue,
    pub publish_prices_loop: EventsLoopMutexWrapped<()>,
    pub instrument_sources_reader: Arc<MyNoSqlDataReaderTcp<InstrumentSourcesEntity>>,
    pub instrument_reader: Arc<MyNoSqlDataReaderTcp<TradingInstrumentNoSqlEntity>>,
    pub price_bridges_settings: Arc<MyNoSqlDataReaderTcp<ProductSettings>>,
    pub markups: Arc<MyNoSqlDataReaderTcp<MarkupProfileNoSqlEntity>>,
    pub bid_ask_publisher: MyServiceBusPublisher<BidAskSbModel>,
    pub settings: Arc<SettingsReader>,
    pub debug_asset: Option<String>,
}

impl AppContext {
    pub async fn new(settings: Arc<SettingsReader>, sc: &ServiceContext) -> Self {
        let debug_asset = if let Ok(debug_asset) = std::env::var("DEBUG_ASSET") {
            Some(debug_asset)
        } else {
            None
        };

        Self {
            bridge_connections: Mutex::new(HashMap::new()),
            publish_prices_loop: EventsLoopMutexWrapped::new("Output Mixer".to_string()),
            bid_ask_to_publish: ToPublishQueue::new(),
            instrument_sources_reader: sc.get_ns_reader().await,
            instrument_reader: sc.get_ns_reader().await,
            price_bridges_settings: sc.get_ns_reader().await,
            markups: sc.get_ns_reader().await,
            bid_ask_publisher: sc.get_sb_publisher(false).await,
            settings,
            debug_asset,
        }
    }
}
