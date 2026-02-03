# Project Log — `sourdough-order-cli`

> Ongoing changelog and coordination notes for manual + Codex edits.

---

## 2025-06-24

### ✅ Environment file cleanup
- Removed `.env` file from Git tracking (`git rm --cached .env`)
- Added `.env` to `.gitignore` to prevent secrets from being committed
- Renamed `.env` → `.env.example` with placeholder values:
  ```env
  ELKS_USER=your_46elks_username
  ELKS_PASS=your_46elks_password
  SMS_MODE=dryrun

  # 🥖 Sourdough Order CLI – Project Log

This log tracks feature decisions, command behavior, and design notes.
Used to coordinate development via ChatGPT (no Codex Agent).

---

## 📅 2025-06-24

### ✅ Implemented Commands (Flat CLI)
These are called as: `cargo run <command>`

- `list-today` → Show today's pickup orders
- `list-all` → Show all saved orders
- `demo` → Load demo data (seeding)
- `reset` → Delete all orders

> CLI is **flat**: no clap, no subcommands. Matches `args[1]`.

---

### 📌 Planned
- `remind` → Dry-run SMS reminder (e.g., “Hej! Ditt bröd är klart…”)
- Add `help` command to show usage

---

## 🧠 Design Notes
- Input is interactive (e.g., `new` might prompt for name/date, but not implemented yet)
- Orders stored in `orders.json`
- Keep logic minimal and readable — no `clap` or modular CLI yet
- ChatGPT acts as planning partner, not code generator
- All edits done manually in `vim` or `nvim` (Termux/WSL)

---

## 🧪 Testing Tips

```bash
cargo run list-today
cargo run demo
cargo run reset
