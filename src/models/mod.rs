use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LimitOrder {
    pub id: i32,
    pub pair: String,
    pub price: u32,
    pub value: u32,
    pub closed: bool,
    pub owner: String,
}
