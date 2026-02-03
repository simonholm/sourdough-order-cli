use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub date: String,
    pub items: Vec<String>,
    pub phone: Option<String>,
}

pub fn save_order(order: &Order, path: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string_pretty(order)?;
    fs::write(path, serialized)?;
    Ok(())
}

pub fn load_orders(path: &str) -> std::io::Result<Vec<Order>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    let orders: Vec<Order> = serde_json::from_str(&content)?;
    Ok(orders)
}
