<!-- README_en.md -->

<p align="right">
  <a href=".github/doc/README_zh.md">🇨🇳 Chinese</a>
</p>

# todo-rs 📝

A minimalist command-line TODO manager written in Rust, designed for Shell integration, no GUI distractions, fast, and elegant.

Supports task management by date, completion status marking, and beautiful terminal prompts, ideal for use with `starship`, `fish`, `zsh`.

## ![preview](.github/preview/td-rs.png)

## ✨ Features

- 📆 Organize tasks by date
- 🔔 Automatically display today's TODO status when starting the terminal
- ⚡ One-click add, complete, remove
- 🖥️ Supports `fish` / `zsh` / `bash` shell prompt integration
- 🧠 Custom `starship` module to display task status
- 💾 Local JSON storage at `~/.config/td-rs/todo.json`

---

## 🚀 Installation

### 1. Clone and Build

```bash
git clone https://github.com/yourname/td-rs
cd td-rs
cargo build --release
```

### 2. Add to PATH

```bash
ln -sf "$PWD/target/release/td" ~/.local/bin/td
```

Ensure `~/.local/bin` is in `$PATH`.

---

## 🧪 Usage Examples

```bash
td add "Write Rust project"
td add "Read paper" --date 2025-08-05

td list                # View today's tasks
td list --date 2025-08-05

td done 1              # Mark as completed
td rm 2                # Delete task

td prompt-today        # Output status: 🔴#1 🟢#3
td count               # Number of incomplete tasks
```

---

## 🪄 Starship Integration

Add the following to your `~/.config/starship.toml`:

```toml
[custom.todo]
command = "td prompt-today"
when = "td prompt-today | grep -q ."
style = "bold red"
format = "[$output]($style) "
```

Effect:

```
🔴#1 🟢#3  ~/projects >
```

You can also bind a shortcut key with `Ctrl+T`:

### Fish:

```fish
function show_todo_list
  td list
end
bind \ct show_todo_list
```

### Zsh:

```zsh
show_todo_list() {
  td list
}
bindkey '^T' show_todo_list
```

---

## 📁 Storage Format

Task data is stored in a local JSON file:

```
~/.config/td-rs/todo.json
```

Example content:

```json
[
  {
    "id": 1,
    "task": "Write README",
    "date": "2025-08-03",
    "done": false
  }
]
```

---

## 📦 TODO (Future Plans)

- [ ] Support tags / project grouping
- [ ] Support task priorities
- [ ] Support recurring tasks (daily, weekly)
- [ ] Enhanced color output
- [ ] Installation script `td install`

---

## 📜 License

MIT License.
