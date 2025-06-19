# 🥖 Sourdough Order CLI

A lightweight CLI tool written in Rust to send SMS updates for bakery orders using the [46elks API](https://46elks.se).  
Built for micro-bakeries that need simple, local, and fast communication with customers.

---

## 🚀 Features

- ✅ Send SMS notifications via 46elks (e.g., order ready for pickup)
- ✅ `.env`-based secrets (no hardcoded credentials)
- ✅ `dryrun` mode for safe testing
- 🛠️ Designed to be extended with CLI flags or order tracking

---

## 🔧 Setup Instructions

### 1. Clone the Repo

```bash
git clone https://github.com/simonholm/sourdough-order-cli.git
cd sourdough-order-cli
