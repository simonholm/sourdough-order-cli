# NOTE (project state)

## Current system (active)

- Web-based ordering (Axum)
- Entry: `cargo run --bin web`
- Customer submits orders via `/order`
- Orders stored in `orders.json`
- Artisan uses dashboard (no notifications)

## SMS module

- File: `src/sms.rs`
- Status: NOT wired to CLI or web flow
- No CLI command for sending SMS
- No SMS triggered on order creation

## History (important)

This repo previously had a CLI-oriented SMS flow
(direct send + dry-run). That interface is no longer
present in `main.rs`.

If SMS is needed again, it must be re-integrated
(e.g. on POST /order or via a separate command).

## Source of truth

- CLI behavior → `src/main.rs`
- Web behavior → `src/bin/web.rs`
- SMS logic → `src/sms.rs` (standalone)

Do not rely on old assumptions; check code.
