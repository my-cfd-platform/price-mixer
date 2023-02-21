use app::AppContext;
use background::{publish_prices_loop::PublishPricesLoop, ConnectionsSynchronizerTimer};
use my_no_sql_tcp_reader::{MyNoSqlDataReader, MyNoSqlTcpConnection};
use my_service_bus_tcp_client::MyServiceBusClient;
use nosql::{DefaultValuesEntity, InstrumentSourcesEntity};
use rust_extensions::MyTimer;
use service_bus_contracts::BidAskSbModel;
use std::{env, sync::Arc};

mod app;
mod background;
mod bridge_config;
mod http;
mod nosql;
mod operations;
mod settings_model;
mod src_feed_client;

#[tokio::main]
async fn main() {
    let (app_name, app_version) = get_app_name_version();
    let settings_reader = Arc::new(settings_model::SettingsReader::new(".my-cfd-platform").await);
    let settings = settings_reader.get_settings().await.settings;

    let logger = my_logger::LOGGER.clone();

    let nosql_connection = MyNoSqlTcpConnection::new(app_name.clone(), settings_reader.clone());
    let instruments_reader: Arc<MyNoSqlDataReader<InstrumentSourcesEntity>> =
        nosql_connection.get_reader().await;
    let defaults_reader: Arc<MyNoSqlDataReader<DefaultValuesEntity>> =
        nosql_connection.get_reader().await;

    let sb_client = Arc::new(MyServiceBusClient::new(
        &app_name,
        &app_version,
        settings_reader.clone(),
        logger.clone(),
    ));
    let bidask_publisher = sb_client.get_publisher::<BidAskSbModel>(true).await;

    let app_ctx = AppContext::new(
        Arc::new(settings),
        logger.clone(),
        instruments_reader,
        defaults_reader,
        bidask_publisher,
        app_name,
        app_version,
    )
    .await;
    let app_ctx = Arc::new(app_ctx);

    app_ctx
        .publish_prices_loop
        .register_event_loop(Arc::new(PublishPricesLoop::new(app_ctx.clone())))
        .await;

    nosql_connection.start().await;
    sb_client.start().await;

    app_ctx
        .publish_prices_loop
        .start(app_ctx.app_states.clone(), app_ctx.logger.clone())
        .await;

    let mut bridge_sync_timer = MyTimer::new(std::time::Duration::from_secs(10));

    bridge_sync_timer.register_timer(
        "BridgeSyncTymer",
        Arc::new(ConnectionsSynchronizerTimer::new(app_ctx.clone())),
    );

    bridge_sync_timer.start(app_ctx.app_states.clone(), app_ctx.logger.clone());
    crate::http::start_up::setup_server(app_ctx.clone());

    app_ctx.app_states.wait_until_shutdown().await;
}

fn get_app_name_version() -> (String, String) {
    let app_name = env::var("APP_NAME");
    let app_name = if app_name.is_ok() {
        app_name.unwrap()
    } else {
        format!("{}-dev", env!("CARGO_PKG_NAME"))
    };
    let app_version = env::var("APP_VERSION");
    let app_version = if app_version.is_ok() {
        app_version.unwrap()
    } else {
        "dev".to_string()
    };

    return (app_name, app_version);
}
