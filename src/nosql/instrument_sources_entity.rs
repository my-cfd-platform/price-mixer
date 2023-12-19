use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("instrument-sources")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentSourcesEntity {
    #[serde(rename = "InstrumentId")]
    pub instrument_id: String,
    #[serde(rename = "SourceId")]
    pub source_id: String,
}

impl InstrumentSourcesEntity {
    pub const PARTITION_KEY: &'static str = "qg";
}
