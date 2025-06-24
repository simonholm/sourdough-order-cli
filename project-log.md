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
