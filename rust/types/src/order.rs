use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Option<Decimal>,
    pub executed_quantity: Decimal,
    pub quote_quantity: Option<Decimal>,
    pub executed_quote_quantity: Decimal,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention: SelfTradePrevention,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
    pub id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub executed_quantity: Decimal,
    pub executed_quote_quantity: Decimal,
    pub price: Decimal,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention: SelfTradePrevention,
    pub post_only: bool,
    pub status: OrderStatus,
    pub created_at: i64,
}

#[derive(
    Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash,
)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "orderType")]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
}

#[derive(
    Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash,
)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    #[default]
    GTC,
    IOC,
    FOK,
}

#[derive(
    Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash,
)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum SelfTradePrevention {
    #[default]
    RejectTaker,
    RejectMaker,
    RejectBoth,
    Allow,
}

#[derive(
    Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash,
)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    Cancelled,
    Expired,
    Filled,
    #[default]
    New,
    PartiallyFilled,
    Triggered,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OrderEvent {
    OrderCancelled,
    OrderExpired,
    OrderFill,
    OrderAccepted,
}

#[derive(
    Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash,
)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum Side {
    #[default]
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOrderPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention: Option<SelfTradePrevention>,
    pub side: Side,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderPayload {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOpenOrdersPayload {
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    #[serde(alias = "e")]
    event: OrderEvent,
    #[serde(alias = "E")]
    event_time: i64,
    #[serde(alias = "s")]
    symbol: String,
    #[serde(alias = "c")]
    client_order_id: String,
    #[serde(alias = "S")]
    side: Side,
    #[serde(alias = "o")]
    order_type: OrderType,
    #[serde(alias = "f")]
    time_in_force: TimeInForce,
    #[serde(alias = "q")]
    qty: Decimal,
    #[serde(alias = "Q")]
    qty_in_quote: Decimal,
    #[serde(alias = "p")]
    price: Decimal,
    #[serde(alias = "P")]
    trigger_price: Decimal,
    #[serde(alias = "X")]
    order_state: OrderStatus,
    #[serde(alias = "i")]
    order_id: String,
    #[serde(alias = "l")]
    filled_qty: String,
    #[serde(alias = "z")]
    executed_qty: Decimal,
    #[serde(alias = "Z")]
    exec_qty_in_quote: Decimal,
    #[serde(alias = "L")]
    filled_price: Decimal,
    #[serde(alias = "m")]
    is_maker: bool,
    #[serde(alias = "n")]
    fee: Decimal,
    #[serde(alias = "N")]
    fee_symbol: String,
    #[serde(alias = "V")]
    self_trade_prevention: SelfTradePrevention,
    #[serde(alias = "T")]
    timestamp: i64,
}
