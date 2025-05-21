use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "result", rename_all = "snake_case")]
pub enum FusionEvent {
    OrderCreated(OrderCreatedEvent),
    OrderBalanceChange(OrderBalanceChangeEvent),
    OrderAllowanceChange(OrderAllowanceChangeEvent),
    OrderInvalid(OrderInvalidEvent),
    OrderCancelled(OrderCancelledEvent),
    OrderFilled(OrderFilledEvent),
    OrderFilledPartially(OrderFilledPartiallyEvent),
    OrderSecretShared(OrderSecretSharedEvent),
    RoundTrip(RoundTripEvent),
    #[serde(rename = "allowedMethods")]
    AllowedMethods(AllowedMethodsEvent),
    Unknown(UnknownEvent),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreatedEvent {
    order_hash: String,
    signature: String,
    order: Order,
    deadline: String,
    auction_start_date: String,
    auction_end_date: String,
    remaining_maker_amount: String,
    quote_id: String,
    extension: String,
    is_maker_contract: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBalanceChangeEvent {
    order_hash: String,
    remaining_maker_amount: String,
    balance: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderAllowanceChangeEvent {
    order_hash: String,
    remaining_maker_amount: String,
    allowance: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderInvalidEvent {
    order_hash: String,
    remaining_maker_amount: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancelledEvent {
    order_hash: String,
    remaining_maker_amount: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderFilledEvent {
    order_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderFilledPartiallyEvent {
    order_hash: String,
    remaining_maker_amount: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderSecretSharedEvent {
    idx: u128,
    secret: String,
    src_immutables: String,
    dst_immutables: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundTripEvent {
    timestamp: u128,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    maker: String,
    maker_asset: String,
    taker_asset: String,
    maker_traits: String,
    salt: String,
    making_amount: String,
    taking_amount: String,
    receiver: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllowedMethodsEvent {
    methods: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnknownEvent {
    event: String,
    result: serde_json::Value,
}