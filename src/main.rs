use std::fs::{self, OpenOptions}; use std::io::Write; use std::path::Path;

use chrono::NaiveDate; use dialoguer::{Input, MultiSelect}; use serde::{Deserialize, Serialize}; use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)] struct Order { id: String, name: String, phone: String, items: Vec<String>, pickup_date: String, status: String, }

fn main() { println!("🥖 Welcome to Sourdough Order CLI - Add New Order\n");

let name: String = Input::new()
    .with_prompt("Customer name")
    .interact_text()
    .unwrap();

let phone: String = Input::new()
    .with_prompt("Phone number (e.g. +46701234567)")
    .interact_text()
    .unwrap();

let items = vec![
    ("Sonjas Ljus Limpa", "sonjas-ljus"),
    ("Kulturråg Limpa", "kulturrag"),
    ("Dinkel med ost Formlimpa", "dinkel-ost"),
    ("Kanelbulle", "kanelbulle"),
    ("Kardemummabulle", "kardemumma"),
];

let item_names: Vec<&str> = items.iter().map(|(name, _)| *name).collect();
let selections = MultiSelect::new()
    .with_prompt("Select product(s)")
    .items(&item_names)
    .interact()
    .unwrap();

let selected_ids: Vec<String> = selections
    .into_iter()
    .map(|i| items[i].1.to_string())
    .collect();

let pickup_date: String = Input::new()
    .with_prompt("Pickup date (YYYY-MM-DD)")
    .validate_with(|input: &String| -> Result<(), &str> {
        if NaiveDate::parse_from_str(input, "%Y-%m-%d").is_ok() {
            Ok(())
        } else {
            Err("Invalid date format")
        }
    })
    .interact_text()
    .unwrap();

let order = Order {
    id: Uuid::new_v4().to_string(),
    name,
    phone,
    items: selected_ids,
    pickup_date,
    status: "pending".to_string(),
};

let path = Path::new("orders.json");
let mut orders: Vec<Order> = if path.exists() {
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
} else {
    vec![]
};

orders.push(order);

let json = serde_json::to_string_pretty(&orders).unwrap();
let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .unwrap();
file.write_all(json.as_bytes()).unwrap();

println!("\n✅ Order saved to orders.json");

}


