use std::collections::HashMap;

use my_no_sql_tcp_reader::MyNoSqlTcpConnectionSettings;
use my_service_bus_tcp_client::MyServiceBusSettings;
use serde_derive::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "MyServiceBusHostPort")]
    pub sb_connection: String,
    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,
    #[serde(rename = "DictionariesMyNoSqlServerReader")]
    pub no_sql_reader: String,
    #[serde(rename = "PriceFeeds")]
    pub bridges_config: HashMap<String, String>,
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
