<!-- README_zh.md -->

<p align="right">
  <a href=".github/doc/README_en.md">🇬🇧 English</a>
</p>

# todo-rs 📝

一个极简的 Rust 编写的命令行 TODO 管理器，主打 Shell 集成、无 GUI 打扰、快速、优雅。

支持按天管理任务、完成状态标记、终端美观提示，适合搭配 `starship`、`fish`、`zsh` 使用。

## ![preview](.github/preview/td-rs.png)

## ✨ 特性

- 📆 以日期为单位组织任务
- 🔔 启动终端时自动显示当天 TODO 状态
- ⚡ 一键添加、完成、移除
- 🖥️ 支持 `fish` / `zsh` / `bash` shell 提示集成
- 🧠 自定义 `starship` 模块显示任务状态
- 💾 本地 JSON 存储于 `~/.config/td-rs/todo.json`

---

## 🚀 安装

### 1. 克隆并编译

```bash
git clone https://github.com/yourname/td-rs
cd td-rs
cargo build --release
```

### 2. 添加到 PATH

```bash
ln -sf "$PWD/target/release/td" ~/.local/bin/td
```

确保 `~/.local/bin` 在 `$PATH` 中。

---

## 🧪 使用示例

```bash
td add "写 Rust 项目"
td add "看论文" --date 2025-08-05

td list                # 查看今天任务
td list --date 2025-08-05

td done 1              # 标记完成
td rm 2                # 删除任务

td prompt-today        # 输出状态：🔴#1 🟢#3
td count               # 未完成任务数量
```

---

## 🪄 Starship 集成

将以下内容加入你的 `~/.config/starship.toml`：

```toml
[custom.todo]
command = "td prompt-today"
when = "td prompt-today | grep -q ."
style = "bold red"
format = "[$output]($style) "
```

效果如下：

```
🔴#1 🟢#3  ~/projects >
```

你也可以通过 `Ctrl+T` 绑定快捷键：

### Fish：

```fish
function show_todo_list
  td list
end
bind \ct show_todo_list
```

### Zsh：

```zsh
show_todo_list() {
  td list
}
bindkey '^T' show_todo_list
```

---

## 📁 存储格式

任务数据存储在本地 JSON 文件：

```
~/.config/td-rs/todo.json
```

示例内容：

```json
[
  {
    "id": 1,
    "task": "写 README",
    "date": "2025-08-03",
    "done": false
  }
]
```

---

## 📦 TODO（未来计划）

- [ ] 支持 tag / 项目分组
- [ ] 支持任务优先级
- [ ] 支持周期性任务（daily, weekly）
- [ ] 彩色输出增强
- [ ] 安装脚本 `td install`

---

## 📜 License

MIT License.
