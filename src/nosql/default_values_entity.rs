use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const LP_DEFAULT: &str = "LiquidityProviderId";
pub const DEFAULT_VALUES_PK: &str = "dv";
pub const DEFAULT_VALUES_TABLE_NAME: &str = "defaultvalues";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultValuesEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "Value")]
    pub value: String,
}

impl MyNoSqlEntity for DefaultValuesEntity {
    const TABLE_NAME: &'static str = DEFAULT_VALUES_TABLE_NAME;

    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn get_row_key(&self) -> &str {
        &self.row_key
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
            .unwrap()
            .unix_microseconds
    }
}
