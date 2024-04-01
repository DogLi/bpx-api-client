use crate::order::{OrderStatus, OrderType, SelfTradePrevention, Side, TimeInForce};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OrderEvent {
    OrderCancelled,
    OrderExpired,
    OrderFill,
    OrderAccepted,
}

// {"data":,"stream":"account.orderUpdate"}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdateStream {
    #[serde(alias = "e")]
    pub event: OrderEvent,
    #[serde(alias = "E")]
    pub event_time: i64,
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "c", default)]
    pub client_order_id: Option<String>,
    #[serde(alias = "S")]
    pub side: Side,
    #[serde(alias = "o")]
    pub order_type: OrderType,
    #[serde(alias = "f")]
    pub time_in_force: TimeInForce,
    #[serde(alias = "q", default)]
    pub qty: Option<Decimal>,
    #[serde(alias = "Q", default)]
    pub qty_in_quote: Option<Decimal>,
    #[serde(alias = "p", default)]
    pub price: Option<Decimal>,
    #[serde(alias = "P", default)]
    pub trigger_price: Option<Decimal>,
    #[serde(alias = "X")]
    pub order_state: OrderStatus,
    #[serde(alias = "i")]
    pub order_id: String,
    #[serde(alias = "l", default)]
    pub filled_qty: Option<Decimal>,
    #[serde(alias = "z")]
    pub executed_qty: Decimal,
    #[serde(alias = "Z")]
    pub exec_qty_in_quote: Decimal,
    #[serde(alias = "L", default)]
    pub filled_price: Option<Decimal>,
    #[serde(alias = "m", default)]
    pub is_maker: Option<bool>,
    #[serde(alias = "n", default)]
    pub fee: Option<Decimal>,
    #[serde(alias = "N", default)]
    pub fee_symbol: Option<String>,
    #[serde(alias = "V")]
    pub self_trade_prevention: SelfTradePrevention,
    #[serde(alias = "T")]
    pub engine_timestamp: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PublicEvent {
    Trade,
    Ticker,
    Kline,
    Depth,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TraderStream {
    #[serde(alias = "e")]
    pub event: PublicEvent,
    #[serde(alias = "E")]
    pub event_timestamp: i64,
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "p")]
    pub price: Decimal,
    #[serde(alias = "q")]
    pub qty: Decimal,
    #[serde(alias = "b")]
    pub buyer_order_id: String,
    #[serde(alias = "a")]
    pub sell_order_id: String,
    #[serde(alias = "t")]
    pub trade_id: u64,
    #[serde(alias = "T")]
    pub engine_timestamp: i64,
    #[serde(alias = "m")]
    pub is_buyer_maker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthStream {
    #[serde(alias = "e")]
    pub event: PublicEvent,
    #[serde(alias = "E")]
    pub timestamp: i64,
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "a")]
    pub asks: Vec<Vec<Decimal>>,
    #[serde(alias = "b")]
    pub bids: Vec<Vec<Decimal>>,
    #[serde(alias = "U")]
    pub first_update_id: u64,
    #[serde(alias = "u")]
    pub final_update_id: u64,
    #[serde(alias = "T")]
    pub engine_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WsStream {
    Depth(DepthStream),
    OrderUpdate(OrderUpdateStream),
    Trader(TraderStream),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_order_stream() {
        let s = r#" {
          "E": 1708179184507397,
          "S": "Bid",
          "T": 1708179184507017,
          "V": "RejectTaker",
          "X": "New",
          "Z": "0",
          "e": "orderAccepted",
          "f": "GTC",
          "i": "111947231035785216",
          "o": "LIMIT",
          "p": "106.84",
          "q": "0.03",
          "s": "SOL_USDC",
          "z": "0"
        }"#;
        let stream: Result<OrderUpdateStream, _> = serde_json::from_str(s);
        println!("{stream:?}");
        assert!(stream.is_ok());
    }
}
