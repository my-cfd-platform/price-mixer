use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use service_sdk::{
    my_no_sql_sdk::reader::MyNoSqlTcpConnectionSettings,
    my_service_bus::client::MyServiceBusSettings,
};

service_sdk::macros::use_settings!();

#[derive(
    my_settings_reader::SettingsModel, SdkSettingsTraits, Serialize, Deserialize, Debug, Clone,
)]
pub struct SettingsModel {
    pub sb_connection: String,
    pub seq_conn_string: String,
    pub no_sql_reader: String,
    pub bridges_config: HashMap<String, String>,
    pub my_telemetry: String,
}

#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;

        read_access.no_sql_reader.clone()
    }
}

#[async_trait::async_trait]

impl MyServiceBusSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;

        read_access.sb_connection.clone()
    }
}

#[async_trait::async_trait]
impl service_sdk::my_telemetry::my_telemetry_writer::MyTelemetrySettings for SettingsReader {
    async fn get_telemetry_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_telemetry.clone()
    }
}

#[async_trait::async_trait]
impl service_sdk::my_logger::my_seq_logger::SeqSettings for SettingsReader {
    async fn get_conn_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.seq_conn_string.clone()
    }
}
