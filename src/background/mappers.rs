use prices_tcp_contracts::{BidAskDataTcpModel, BidAskDateTimeTcpModel};
use service_bus_contracts::BidAskSbModel;

pub fn map_bid_ask_to_sb_model(tcp_model: &BidAskDataTcpModel) -> BidAskSbModel {
    let date_time = match tcp_model.datetime {
        BidAskDateTimeTcpModel::Source(date) => date,
        BidAskDateTimeTcpModel::Our(date) => date,
        BidAskDateTimeTcpModel::Generated(date) => date,
    };

    BidAskSbModel {
        id: tcp_model.instrument_id.to_string(),
        date_time_unix_milis: date_time.timestamp_millis() as u64,
        bid: tcp_model.bid,
        ask: tcp_model.ask,
    }
}
