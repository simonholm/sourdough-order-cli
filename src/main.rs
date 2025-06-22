use std::env; use std::fs::{self, OpenOptions}; use std::io::Write; use std::path::Path;

use chrono::{Datelike, Local, NaiveDate}; use dialoguer::{Input, MultiSelect}; use serde::{Deserialize, Serialize}; use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)] struct Order { id: String, name: String, phone: String, items: Vec<String>, pickup_date: String, status: String, }

fn main() { let args: Vec<String> = env::args().collect();

if args.len() > 1 && args[1] == "list-today" {
    list_today();
    return;
}

println!("\n🥖 Sourdough Order CLI v0.1.0 - Add New Order\n");

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

println!("(Use ↑/↓ to move, Space to select, Enter to confirm)\n");

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

let path = Path::new("src/orders.json");
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

fn list_today() { let path = Path::new("src/orders.json"); if !path.exists() { println!("No orders.json file found."); return; }

let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
let orders: Vec<Order> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);

let today = Local::now().date_naive();
let mut found = false;

println!("\n📋 Orders for {}:\n", today);
for order in orders {
    if let Ok(date) = NaiveDate::parse_from_str(&order.pickup_date, "%Y-%m-%d") {
        if date == today {
            found = true;
            println!("- {}: {} ({})", order.name, order.items.join(", "), order.phone);
        }
    }
}

if !found {
    println!("No orders found for today.");
}

}


