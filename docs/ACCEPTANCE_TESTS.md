# Acceptance Tests – Sourdough Order CLI

Each test case below describes a scenario that must pass for the CLI to be considered functionally complete.

---

## ✅ AT-001: Load and Display Today's Orders

**Given** a file `src/orders.json`  
**And** it contains at least one order with today's date (`pickup_date == 2025-06-22`)  
**When** the user runs:  
```bash
cargo run -- --today
```

**Then** the output must display:
* Customer name
* Phone number
* Item list
* Matching date

---

## ✅ AT-002: Filter Orders by Status

**Given** orders with various statuses (`pending`, `ready`)  
**When** the user runs:  
```bash
cargo run -- --status pending
```

**Then** only `pending` orders are shown.

---

## ✅ AT-003: Show Upcoming Orders

**Given** orders scheduled for future dates  
**When** the user runs:  
```bash
cargo run -- --next 3
```

**Then** orders within the next 3 days (inclusive) are listed.

---

## ✅ AT-004: Handle Missing File Gracefully

**Given** the specified file path does not exist  
**When** the user runs:  
```bash
cargo run -- --file nonexistent.json
```

**Then** a clear error message is printed:  
`Error: File 'nonexistent.json' not found.`

---

## ✅ AT-005: Dry-Run Mode Prints Simulated Output

**Given** valid order data  
**When** the user runs:  
```bash
cargo run -- --today --dry-run
```

**Then** the CLI outputs the order summary without modifying any data files.

---

## ✅ AT-006: Malformed JSON Handling

**Given** the file contains invalid JSON  
**When** the CLI is run  
**Then** it prints a clear error:  
`Error: Failed to parse JSON (line X, column Y)`

---

*Last updated: 2025-06-22*
