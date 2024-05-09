use cfd_engine_sb_contracts::BidAskSbModel;

use crate::app::ToPublishItem;

pub fn map_bid_ask_to_sb_model(model: ToPublishItem) -> BidAskSbModel {
    return BidAskSbModel {
        id: model.bid_ask.instrument_id,
        date_time_unix_milis: model.bid_ask.date_time.get_date_time().unix_microseconds as u64,
        bid: model.bid_ask.bid,
        ask: model.bid_ask.ask,
        base: model.instrument.base.to_string(),
        quote: model.instrument.quote.to_string(),
    };
}

pub fn map_bid_ask_to_sb_model_with_markup(
    model: ToPublishItem,
    markup_bid: i32,
    markup_ask: i32,
) -> BidAskSbModel {
    if markup_bid == 0 && markup_ask == 0 {
        return BidAskSbModel {
            id: model.bid_ask.instrument_id,
            date_time_unix_milis: model.bid_ask.date_time.get_date_time().unix_microseconds as u64,
            bid: model.bid_ask.bid,
            ask: model.bid_ask.ask,
            base: model.instrument.base.to_string(),
            quote: model.instrument.quote.to_string(),
        };
    }

    let multiplier = 1.0 / i64::pow(10, model.instrument.digits as u32) as f64;

    return BidAskSbModel {
        id: model.bid_ask.instrument_id,
        date_time_unix_milis: model.bid_ask.date_time.get_date_time().unix_microseconds as u64,
        bid: model.bid_ask.bid + multiplier * markup_bid as f64,
        ask: model.bid_ask.ask + multiplier * markup_ask as f64,
        base: model.instrument.base.to_string(),
        quote: model.instrument.quote.to_string(),
    };
}
