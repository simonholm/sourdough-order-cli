use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub items: Vec<String>,
    pub pickup_date: String,
    pub status: String,
}

pub fn load_orders(path: &str) -> std::io::Result<Vec<Order>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    let orders: Vec<Order> = serde_json::from_str(&content).unwrap_or_default();
    Ok(orders)
}

pub fn save_orders(path: &str, orders: &[Order]) -> std::io::Result<()> {
    let serialized = serde_json::to_string_pretty(orders)?;
    fs::write(path, serialized)
}
