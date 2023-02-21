use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const PRICE_SOURCES_TABLE_NAME: &str = "instrument-sources";
pub const INSTRUMENT_SOURCES_PK: &str = "qg";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentSourcesEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "InstrumentId")]
    pub instrument_id: String,
    #[serde(rename = "SourceId")]
    pub source_id: String,
}

impl MyNoSqlEntity for InstrumentSourcesEntity {
    const TABLE_NAME: &'static str = PRICE_SOURCES_TABLE_NAME;

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
