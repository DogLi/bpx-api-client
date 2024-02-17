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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdateStream {
    #[serde(alias = "e")]
    pub event: OrderEvent,
    #[serde(alias = "E")]
    pub event_time: i64,
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "c")]
    pub client_order_id: String,
    #[serde(alias = "S")]
    pub side: Side,
    #[serde(alias = "o")]
    pub order_type: OrderType,
    #[serde(alias = "f")]
    pub time_in_force: TimeInForce,
    #[serde(alias = "q")]
    pub qty: Decimal,
    #[serde(alias = "Q")]
    pub qty_in_quote: Decimal,
    #[serde(alias = "p")]
    pub price: Decimal,
    #[serde(alias = "P")]
    pub trigger_price: Decimal,
    #[serde(alias = "X")]
    pub order_state: OrderStatus,
    #[serde(alias = "i")]
    pub order_id: String,
    #[serde(alias = "l")]
    pub filled_qty: String,
    #[serde(alias = "z")]
    pub executed_qty: Decimal,
    #[serde(alias = "Z")]
    pub exec_qty_in_quote: Decimal,
    #[serde(alias = "L")]
    pub filled_price: Decimal,
    #[serde(alias = "m")]
    pub is_maker: bool,
    #[serde(alias = "n")]
    pub fee: Decimal,
    #[serde(alias = "N")]
    pub fee_symbol: String,
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

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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
