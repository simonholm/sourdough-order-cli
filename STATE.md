# Project State

- Core domain modules live in `src/order.rs` (order model + JSON load/save) and `src/menu.rs` (static menu catalog); they are UI-agnostic and ready for reuse.
- `src/main.rs` is the current operator-facing CLI, combining argument dispatch, prompts, validation, and file I/O; it duplicates order/menu structures, assumes a writable `orders.json` in the working directory, and skips SMS sending.
- Error handling is permissive: malformed or missing JSON is tolerated by falling back to empty lists, and validation is limited to basic date checks and required fields.
- SMS support exists in `src/sms.rs` with environment-based credentials and a `dry_run` mode but is not wired into the CLI flow.
- Near-term priorities from project notes: integrate scratchpad menu/order modules to eliminate duplication, adopt `clap` subcommands (`new`, `list-today`, `list-all`, `demo`, `reset`), add order status tracking and richer date filtering, and start adding tests to stabilize future changes.
- A future GUI can reuse the domain modules by introducing a programmatic order-placement entry point that bypasses prompts while preserving the existing JSON persistence format.
