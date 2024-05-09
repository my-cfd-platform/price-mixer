use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone)]
pub struct PriceMixerBidAskModel {
    pub id: String,
    pub bid: f64,
    pub ask: f64,
    pub volume: f64,
    pub date: DateTimeAsMicroseconds,
    pub base: String,
    pub quote: String,
}
