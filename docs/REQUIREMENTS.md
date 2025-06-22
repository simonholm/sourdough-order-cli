# Requirements – Sourdough Order CLI

## Functional Requirements

### FR1 – List Today’s Orders
- The system must support filtering orders with `pickup_date` matching the current date.

### FR2 – Load JSON Order Data
- Orders must be read from a specified JSON file (e.g., `orders.json`).
- File path should be configurable or default to a known location (e.g., `src/orders.json`).

### FR3 – Filter by Status
- Users must be able to filter orders by status (e.g., `pending`, `completed`, `cancelled`).

### FR4 – Display Order Summary
- Output must include: customer name, phone number, pickup date, and item list.

### FR5 – Dry-Run Mode
- The system should support a dry-run flag (`--dry-run`) that simulates behavior without modifying data.

### FR6 – Command-Line Arguments
- Users must be able to pass parameters like:
  - `--today`
  - `--next <n>` (e.g., next 3 days)
  - `--status pending`
  - `--file <path>`

### FR7 – Error Handling
- The CLI must show readable messages for:
  - File not found
  - Malformed JSON
  - Missing required fields

---

## Non-Functional Requirements

### NFR1 – Portability
- The CLI should run on Linux and WSL environments using only Rust standard libraries or minimal dependencies.

### NFR2 – Readability
- Output should be clear and structured, suitable for TUI-based viewing or log export.

### NFR3 – Maintainability
- Code should be modular, with clear separation between CLI parsing, data loading, and filtering logic.

### NFR4 – Testability
- Core logic should be testable via unit tests using example input files.

---

*Last updated: 2025-06-22*

