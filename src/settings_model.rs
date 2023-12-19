use serde::{Deserialize, Serialize};
service_sdk::macros::use_settings!();

#[derive(
    my_settings_reader::SettingsModel,
    AutoGenerateSettingsTraits,
    SdkSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct SettingsModel {
    pub my_sb_tcp_host_port: String,
    pub seq_conn_string: String,
    pub my_no_sql_tcp_reader: String,
    pub my_telemetry: String,
}
