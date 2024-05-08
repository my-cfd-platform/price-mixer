use cfd_engine_sb_contracts::BidAskSbModel;
use prices_tcp_contracts::{BidAskDataTcpModel, BidAskDateTimeTcpModel};

use crate::models::PriceMixerBidAskModel;

pub fn map_bid_ask_to_sb_model(model: PriceMixerBidAskModel) -> BidAskSbModel {
    return BidAskSbModel {
        id: model.id.to_string(),
        date_time_unix_milis: model.date.unix_microseconds as u64,
        bid: model.bid,
        ask: model.ask,
        base: model.base,
        quote: model.quote,
    };
}

pub fn map_bid_ask_to_sb_model_with_markup(
    model: PriceMixerBidAskModel,
    markup_bid: i32,
    markup_ask: i32,
    digits: i32,
) -> BidAskSbModel {
    if markup_bid == 0 && markup_ask == 0 {
        return BidAskSbModel {
            id: model.id.to_string(),
            date_time_unix_milis: model.date.unix_microseconds as u64,
            bid: model.bid,
            ask: model.ask,
            base: model.base,
            quote: model.quote,
        };
    }

    let multiplier = 1.0 / i64::pow(10, digits as u32) as f64;

    return BidAskSbModel {
        id: model.id.to_string(),
        date_time_unix_milis: model.date.unix_microseconds as u64,
        bid: model.bid + multiplier * markup_bid as f64,
        ask: model.ask + multiplier * markup_ask as f64,
        base: model.base,
        quote: model.quote,
    };
}

pub fn map_tcp_to_inner(
    model: BidAskDataTcpModel,
    base: &str,
    quote: &str,
) -> PriceMixerBidAskModel {
    let date = match model.date_time {
        BidAskDateTimeTcpModel::Source(date) => date,
        BidAskDateTimeTcpModel::Our(date) => date,
        BidAskDateTimeTcpModel::Generated(date) => date,
    };

    PriceMixerBidAskModel {
        id: model.instrument_id,
        bid: model.bid,
        ask: model.ask,
        volume: model.volume,
        date,
        base: base.to_string(),
        quote: quote.to_string(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let value = i64::pow(10, 2);

        print!("{}", 1.0 / value as f64)
    }
}
