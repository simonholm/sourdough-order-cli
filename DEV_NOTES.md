# DEV_NOTES.md — sourdough-order-cli

This file documents the current state of the project, what’s been prototyped, and what’s coming next.

---

## 🧠 Project Summary

A Rust-based command-line and (future) web interface for managing sourdough bread orders.  
The system should:
- Let customers order via a mobile-friendly UI (linkable from Instagram)
- Let the artisan view and print pickup lists
- Store orders as JSON (or SQLite later)
- Share menu/order logic across CLI and web UI

---

## 📁 Key Structure

| Path                      | Purpose                             |
|---------------------------|-------------------------------------|
| `src/main.rs`             | CLI entry point (routing TBD)       |
| `src/sms.rs`              | SMS integration                     |
| `scratchpad/menu.rs`      | Static menu, returns (name, id)     |
| `scratchpad/order.rs`     | `Order` struct + save/load logic    |
| `docs/`                   | User stories, requirements          |
| `meta/`                   | Formatting tips, Codex checklist    |
| `orders.json`             | Possibly sample or legacy format    |

---

## 🛠️ Scratchpad Use

`scratchpad/` holds experimental or reusable modules that aren’t wired into the app yet.  
This includes:
- Menu system
- Order file logic

Once stable, they will be moved into `src/`.

---

## 🔜 Next Options

1. 🧪 Test modules via `test_menu.rs`, `test_order.rs`
2. 🧵 Create `src/cli.rs` using `clap`, link to `menu` + `order`
3. 🌐 Start `src/web/` using `axum + askama`
4. 📚 Connect everything via `main.rs`
5. 🤖 Write `AGENTS.md` for Codex task guidance

---

## 🧩 Codex Agent Notes (if used)

- Should not modify scratchpad directly unless told
- Must use Rust only (no Flask or JS)
- Must write to `orders/YYYY-MM-DD.json`

---
