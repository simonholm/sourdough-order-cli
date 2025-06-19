use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::env;
use std::error::Error;

/// Sends an SMS using 46elks API.
/// 
/// `dry_run = true` will only print the message, not send it.
pub fn send_sms(to: &str, message: &str, from: &str, dry_run: bool) -> Result<(), Box<dyn Error>> {
    if dry_run {
        println!("📦 [DRY-RUN] Would send SMS:");
        println!("From: {}\nTo: {}\nMessage: {}", from, to, message);
        return Ok(());
    }

    let user = env::var("ELKS_USER")?;
    let pass = env::var("ELKS_PASS")?;

    let client = Client::new();
    let res = client
        .post("https://api.46elks.com/a1/SMS")
        .basic_auth(user, Some(pass))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(format!("from={}&to={}&message={}", from, to, message))
        .send()?;

    if res.status().is_success() {
        println!("✅ SMS sent!");
        let body = res.text()?;
        println!("{body}");
    } else {
        eprintln!("❌ SMS failed: {}", res.status());
        let body = res.text()?;
        eprintln!("{body}");
    }

    Ok(())
}
