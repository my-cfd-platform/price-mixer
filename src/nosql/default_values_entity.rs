use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("defaultvalues")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultValuesEntity {
    #[serde(rename = "Value")]
    pub value: String,
}

impl DefaultValuesEntity {
    pub const LP_DEFAULT_ROW_KEY: &'static str = "LiquidityProviderId";
    pub const DEFAULT_VALUES_PARTITION_KEY: &'static str = "dv";
}
