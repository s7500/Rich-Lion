use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LimitOrder {
    pub id: i32,
    pub pair: String,
    pub price: i32,
    pub value: i32,
    pub closed: bool,
    pub owner: String,
}
