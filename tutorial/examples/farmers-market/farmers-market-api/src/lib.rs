use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A purchasable product. Not available for purchase if quantity is 0.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}
