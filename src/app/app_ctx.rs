use super::BridgeConnection;
use crate::{
    settings_model::SettingsModel, DefaultValuesEntity,
    InstrumentSourcesEntity,
};
use my_logger::MyLogger;
use my_no_sql_tcp_reader::MyNoSqlDataReader;
use my_service_bus_abstractions::publisher::MyServiceBusPublisher;
use prices_tcp_contracts::BidAskDataTcpModel;
use rust_extensions::{events_loop::EventsLoop, AppStates};
use service_bus_contracts::BidAskSbModel;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub bridge_connections: Mutex<HashMap<String, BridgeConnection>>,
    pub bid_ask_to_publish: Mutex<Vec<BidAskDataTcpModel>>,
    pub publish_prices_loop: EventsLoop<()>,
    pub logger: Arc<MyLogger>,
    pub app_states: Arc<AppStates>,
    pub settings: Arc<SettingsModel>,
    pub instruments_reader: Arc<MyNoSqlDataReader<InstrumentSourcesEntity>>,
    pub defaults_reader: Arc<MyNoSqlDataReader<DefaultValuesEntity>>,
    pub bidask_publisher: MyServiceBusPublisher<BidAskSbModel>,
}

impl AppContext {
    pub async fn new(
        settings: Arc<SettingsModel>,
        logger: Arc<MyLogger>,
        instruments_reader: Arc<MyNoSqlDataReader<InstrumentSourcesEntity>>,
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
            instruments_reader,
            defaults_reader,
            bidask_publisher,
        }
    }
}
