# AGENTS.md — Codex Agent Instructions for `sourdough-order-cli`

These instructions guide the behavior of Codex Agent when operating on this repository.

---

## 🎯 Scope and Purpose

This project implements a CLI and (future) web UI for managing sourdough bread orders in Rust.  
Codex Agent may assist with:

- CLI interface scaffolding (using `clap`)
- Data handling via JSON or files
- Web form backend (`axum`, `askama`, `serde`)
- Tests for modules (`menu`, `order`, etc.)
- Input sanitization, validation, or menu formatting

---

## 📦 Directory Rules

| Directory         | Purpose                                        | Agent Permission |
|-------------------|------------------------------------------------|------------------|
| `src/`            | Production CLI and backend logic               | ✅ Modify/add     |
| `scratchpad/`     | Staging area for modules and ideas             | ⚠️ Modify *only* if instructed |
| `docs/`           | Formal requirements and acceptance tests       | 🛑 Do not modify  |
| `meta/`           | Formatting tips and Codex prompts              | 🛑 Do not modify  |
| `orders/`         | Finalized order outputs                        | ✅ Can write/read |
| `orders.json`     | Legacy sample                                  | ⚠️ Read-only      |

---

## 🧠 Behavior Rules

- Do **not** overwrite or remove scratchpad files unless explicitly instructed
- Prefer small commits and semantic PR titles (e.g. `feat(cli): add print-menu command`)
- Output changes in human-readable Markdown diffs or commit-ready code
- Include helpful comments if code touches anything non-obvious
- Avoid Flask, JavaScript, or Python-based web suggestions — this is a **Rust-only project**

---

## 🧪 Validation & Tests

- New modules must compile with `cargo build`
- Menu modules must return Vec<(name, slug)>
- Order modules must save/load consistent JSON
- CLI and web must default to Saturday pickup unless otherwise configured

---

## 📌 Codex Commit Conventions

When submitting PRs, Codex Agent should:

- Reference the original intent (e.g. *“Implements part of task X from DEV_NOTES.md”*)
- Mention whether tests were added (if applicable)
- Avoid “big bang” changes unless clearly validated

---


