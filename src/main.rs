mod sms;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let to = "+46701234567";
    let message = "Hej! Ditt bröd är klart för avhämtning 🥖";
    let from = "Bageriet";

    let sms_mode = env::var("SMS_MODE").unwrap_or_else(|_| "dryrun".to_string());
    let dry_run = sms_mode != "live";

    if let Err(e) = sms::send_sms(to, message, from, dry_run) {
        eprintln!("❌ SMS error: {e}");
    }
}
