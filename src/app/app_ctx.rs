use super::BridgeConnection;
use crate::{
    models::PriceMixerBidAskModel, settings_model::SettingsReader, nosql::{InstrumentSourcesEntity, DefaultValuesEntity},
};
use cfd_engine_sb_contracts::BidAskSbModel;
use my_nosql_contracts::TradingInstrumentNoSqlEntity;
use rust_extensions::events_loop::EventsLoop;
use service_sdk::{
    my_no_sql_sdk::reader::MyNoSqlDataReader,
    my_service_bus::abstractions::publisher::MyServiceBusPublisher, ServiceContext,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct AppContext {
    pub bridge_connections: Mutex<HashMap<String, BridgeConnection>>,
    pub bid_ask_to_publish: Mutex<Vec<PriceMixerBidAskModel>>,
    pub publish_prices_loop: EventsLoop<()>,
    pub instrument_sources_reader:
        Arc<dyn MyNoSqlDataReader<InstrumentSourcesEntity> + Send + Sync>,
    pub instrument_reader: Arc<dyn MyNoSqlDataReader<TradingInstrumentNoSqlEntity> + Send + Sync>,
    pub defaults_reader: Arc<dyn MyNoSqlDataReader<DefaultValuesEntity> + Send + Sync>,
    pub bidask_publisher: MyServiceBusPublisher<BidAskSbModel>,
    pub settings: Arc<SettingsReader>
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
            bidask_publisher: sc.get_sb_publisher(false).await,
            settings
        }
    }
}
