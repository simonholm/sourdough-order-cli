use axum::{Form, Router, extract::State, response::Html, routing::get};
use chrono::Local;
use serde::Deserialize;
use sourdough_order_cli::order::{Order, load_orders, save_orders};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    orders_path: Arc<str>,
}

#[derive(Deserialize)]
struct OrderForm {
    name: String,
    phone: String,
    product: String,
    qty: u32,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        orders_path: Arc::from("orders.json"),
    };

    let app = Router::new()
        .route("/order", get(show_order_form).post(submit_order))
        .route("/artisan", get(show_artisan_view))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .expect("failed to bind to 0.0.0.0:80");

    println!("Web frontend running at http://localhost:80/order");

    axum::serve(listener, app).await.expect("server error");
}

async fn show_order_form() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Sourdough Order</title>
  <style>
    :root { color-scheme: light; }
    body {
      margin: 0;
      font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
      background: #f7f4ef;
      color: #2e261e;
    }
    .wrap {
      max-width: 520px;
      margin: 0 auto;
      padding: 1rem;
    }
    .card {
      background: #fff;
      border-radius: 14px;
      padding: 1rem;
      box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
    }
    h1 { margin-top: 0; font-size: 1.4rem; }
    label { display: block; margin-top: 0.75rem; font-weight: 600; }
    input {
      width: 100%;
      box-sizing: border-box;
      border: 1px solid #d5c7b6;
      border-radius: 10px;
      padding: 0.7rem;
      font-size: 1rem;
      margin-top: 0.35rem;
      background: #fffdfa;
    }
    button {
      margin-top: 1rem;
      width: 100%;
      border: 0;
      border-radius: 10px;
      padding: 0.8rem;
      font-size: 1rem;
      font-weight: 700;
      background: #8b5e34;
      color: #fff;
    }
    .muted { font-size: 0.9rem; color: #6d6054; }
    a { color: #8b5e34; }
  </style>
</head>
<body>
  <main class="wrap">
    <section class="card">
      <h1>Place an order</h1>
      <p class="muted">Simple mobile form for bakery pickup orders.</p>
      <form method="post" action="/order">
        <label for="name">Name</label>
        <input id="name" name="name" required>

        <label for="phone">Phone</label>
        <input id="phone" name="phone" required>

        <label for="product">Product</label>
        <input id="product" name="product" placeholder="e.g. Sonjas Ljus Limpa" required>

        <label for="qty">Qty</label>
        <input id="qty" name="qty" type="number" min="1" value="1" required>

        <button type="submit">Submit Order</button>
      </form>
      <p class="muted"><a href="/artisan">View artisan dashboard</a></p>
    </section>
  </main>
</body>
</html>"#,
    )
}

async fn submit_order(State(state): State<AppState>, Form(form): Form<OrderForm>) -> Html<String> {
    if form.qty == 0 {
        return Html(
            "<h2>Invalid quantity</h2><p>Quantity must be at least 1.</p><p><a href=\"/order\">Back</a></p>"
                .to_string(),
        );
    }

    let mut orders = load_orders(&state.orders_path).unwrap_or_default();

    let order = Order {
        id: Uuid::new_v4().to_string(),
        name: form.name,
        phone: form.phone,
        items: vec![format!("{} x{}", form.product, form.qty)],
        pickup_date: Local::now().date_naive().to_string(),
        status: "pending".to_string(),
    };

    orders.push(order);

    let result = save_orders(&state.orders_path, &orders);

    match result {
        Ok(_) => Html(
            "<h2>Order received.</h2><p><a href=\"/order\">New order</a> | <a href=\"/artisan\">View orders</a></p>".to_string(),
        ),
        Err(err) => Html(format!(
            "<h2>Failed to save order</h2><p>{}</p><p><a href=\"/order\">Back</a></p>",
            err
        )),
    }
}

async fn show_artisan_view(State(state): State<AppState>) -> Html<String> {
    let orders = load_orders(&state.orders_path).unwrap_or_default();

    if orders.is_empty() {
        return Html(
            "<h2>No orders yet</h2><p><a href=\"/order\">Create first order</a></p>".to_string(),
        );
    }

    let mut rows = String::new();
    for order in orders {
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            escape_html(&order.pickup_date),
            escape_html(&order.name),
            escape_html(&order.phone),
            escape_html(&order.items.join(", ")),
            escape_html(&order.status),
        ));
    }

    Html(format!(
        "<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
  <title>Artisan Orders</title>
  <style>
    body {{ font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, sans-serif; margin: 0; background: #f7f4ef; color: #2e261e; }}
    .wrap {{ max-width: 880px; margin: 0 auto; padding: 1rem; }}
    .card {{ background: #fff; border-radius: 14px; padding: 1rem; box-shadow: 0 8px 24px rgba(0,0,0,.08); overflow-x: auto; }}
    table {{ width: 100%; border-collapse: collapse; min-width: 620px; }}
    th, td {{ text-align: left; border-bottom: 1px solid #ece2d7; padding: 0.55rem; }}
    th {{ background: #faf4ec; }}
    a {{ color: #8b5e34; }}
  </style>
</head>
<body>
  <main class=\"wrap\">
    <section class=\"card\">
      <h1>Artisan Dashboard</h1>
      <p><a href=\"/order\">New Order</a></p>
      <table>
        <thead>
          <tr><th>Date</th><th>Name</th><th>Phone</th><th>Items</th><th>Status</th></tr>
        </thead>
        <tbody>{}</tbody>
      </table>
    </section>
  </main>
</body>
</html>",
        rows
    ))
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
