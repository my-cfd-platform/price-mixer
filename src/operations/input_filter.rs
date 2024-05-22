use my_nosql_contracts::{InstrumentSourcesEntity, TradingInstrumentNoSqlEntity};
use prices_tcp_contracts::BidAskDataTcpModel;

use crate::app::AppContext;

use std::sync::Arc;

pub async fn process(app: &Arc<AppContext>, bid_ask: BidAskDataTcpModel, src: &str) {
    let Some(instrument) = app
        .instrument_reader
        .get_entity(
            TradingInstrumentNoSqlEntity::generate_partition_key(),
            &bid_ask.instrument_id,
        )
        .await
    else {
        return;
    };
    /*
    let Some(instrument_src) = app
        .instrument_sources_reader
        .get_entity(
            InstrumentSourcesEntity::PARTITION_KEY,
            &bid_ask.instrument_id,
        )
        .await
    else {
        return;
    };


    if !rust_extensions::str_utils::compare_strings_case_insensitive(
        instrument_src.source_id.as_str(),
        src,
    ) {
        return;
    }
     */

    app.bid_ask_to_publish.publish(instrument, bid_ask).await;

    app.publish_prices_loop.send(());
}

/*
pub async fn can_we_send_quote(app: &Arc<AppContext>, instrument_id: &str, source: &str) -> bool {
    let quote_map: Option<Arc<InstrumentSourcesEntity>> = app
        .instrument_sources_reader
        .get_entity(InstrumentSourcesEntity::PARTITION_KEY, instrument_id)
        .await;

    if quote_map.is_some() {
        return quote_map.unwrap().source_id == source;
    }

    let default_lp: Option<Arc<DefaultValuesEntity>> = app
        .defaults_reader
        .get_entity(
            DefaultValuesEntity::DEFAULT_VALUES_PARTITION_KEY,
            DefaultValuesEntity::LP_DEFAULT_ROW_KEY,
        )
        .await;

    if default_lp.is_some() {
        return default_lp.unwrap().value == source;
    }

    return false;
}

*/
