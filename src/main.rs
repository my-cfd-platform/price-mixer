use std::{sync::Arc, time::Duration};

use app::AppContext;
use background::{publish_prices_loop::PublishPricesLoop, ConnectionsSynchronizerTimer};
use settings_model::SettingsReader;

mod app;
mod background;
mod bridge_config;
mod models;
mod nosql;
mod operations;
mod settings_model;
mod src_feed_client;

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".my-cfd-platform").await;
    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;
    let app_context = Arc::new(AppContext::new(settings_reader.clone(), &service_context).await);

    app_context
        .publish_prices_loop
        .register_event_loop(Arc::new(PublishPricesLoop::new(app_context.clone())))
        .await;

    service_context.register_background_job(
        Duration::from_secs(10),
        "BridgeSyncTimer",
        Arc::new(ConnectionsSynchronizerTimer::new(app_context.clone())),
    );

    app_context
        .publish_prices_loop
        .start(
            service_context.app_states.clone(),
            service_sdk::my_logger::LOGGER.clone(),
        )
        .await;
    service_context.start_application().await;
}
