# ✍️ ChatGPT Formatting Tips for Mobile Copy-Paste

When requesting full file outputs (like `.md`, `.sh`, `.py`) from ChatGPT, especially when working on mobile, avoid broken formatting caused by triple backtick blocks.

---

## ✅ Recommended Prompt (Safe for Mobile)

> Please give me the entire file as a single Markdown block,  
> with all inner code indented using 4 spaces instead of triple backticks.  
> I want one continuous block I can copy-paste on mobile without it splitting into separate widgets.

---

## ✅ Why this works

- Indented code (`4 spaces`) is standard Markdown
- Prevents ChatGPT from splitting into "Copy code" widgets
- Keeps all content fully embedded in one block
- Works reliably on mobile apps and web UI

---

## 🧠 Tip

Use this prompt when requesting:
- Shell scripts
- Markdown files
- Multi-block outputs like README + checklist
- Anything where nested ` ```bash ` blocks might cause layout issues

Keep this file as part of your `meta/` folder for reference.
