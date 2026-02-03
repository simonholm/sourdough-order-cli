# 🛑 Codex Agent Safety Checklist

Codex Agent (via ChatGPT) does not track or commit its changes.  
Use this checklist before and after invoking Codex to prevent accidental overwrites or confusion.

---

## ✅ BEFORE using Codex Agent

Run these commands to inspect the current state of your local repository:

    git status        # Ensure working directory is clean
    git diff --stat   # See if any files were touched
    git log --oneline # Review recent commits

---

## ✅ AFTER using Codex Agent

Immediately inspect the changes Codex may have made:

    git diff          # See what changed in detail
    git diff --stat   # Confirm scope of changes

---

## 🧯 If needed — undo or finalize:

    git restore .                    # Undo all local uncommitted changes
    git commit -m "Codex edit: ..." # If changes look good
    git push                         # Sync to GitHub

---

## ❌ Codex does NOT:

- Track what it edits  
- Auto-commit changes  
- Warn you before overwriting  
- Show a diff or change history  
- Remember what it did last session  

---

## 🧠 Best Practices

- Always **commit before using Codex**, even for small edits  
- Treat Codex as **write-unsafe** unless paired with Git inspection  
- Keep this checklist nearby (`meta/codex-checklist.md`) for future use
