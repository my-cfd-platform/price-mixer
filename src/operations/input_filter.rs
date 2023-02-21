use prices_tcp_contracts::BidAskDataTcpModel;

use crate::{
    app::AppContext,
    nosql::{DEFAULT_VALUES_PK, INSTRUMENT_SOURCES_PK, LP_DEFAULT},
    DefaultValuesEntity, InstrumentSourcesEntity,
};

use std::sync::Arc;

pub async fn process(app: &Arc<AppContext>, bid_ask: BidAskDataTcpModel, src: &str) {
    if can_we_send_quote(app, &bid_ask.instrument_id, src).await {
        return;
    }

    let mut write_access = app.bid_ask_to_publish.lock().await;
    write_access.push(bid_ask);
    app.publish_prices_loop.send(());
}

pub async fn can_we_send_quote(app: &Arc<AppContext>, instrument_id: &str, source: &str) -> bool {
    let quote_map: Option<Arc<InstrumentSourcesEntity>> = app
        .instruments_reader
        .get_entity(INSTRUMENT_SOURCES_PK, instrument_id)
        .await;

    if quote_map.is_some() {
        return quote_map.unwrap().source_id == source;
    }

    let default_lp: Option<Arc<DefaultValuesEntity>> = app
        .defaults_reader
        .get_entity(DEFAULT_VALUES_PK, LP_DEFAULT)
        .await;

    if default_lp.is_some() {
        return default_lp.unwrap().value == source;
    }

    return false;
}
