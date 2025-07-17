# 🦀 俄罗斯方块（Rust 版）

一条命令即可运行的极简俄罗斯方块，纯 Rust + macroquad 实现。

---

## 🎯 项目目标
- 用 **Rust** 写一款 **可在本地 & Web 运行** 的俄罗斯方块  
- **≤200 行** 核心代码，保持可读性  
- 零依赖部署：`cargo run` 或浏览器打开

---

## 🧰 技术栈
| 层级 | 选型 | 说明 |
|---|---|---|
| 语言 | **Rust 1.75+** | 编译型，零成本抽象 |
| 图形 / 事件 | [**macroquad**](https://github.com/not-fl3/macroquad) | 轻量 2D，跨平台（Win / macOS / Linux / Web） |
| 构建 | Cargo | 自带依赖管理 |
| 部署 | `cargo run` 或 `cargo web start` | 原生 / WASM 一键 |

---

## 🛠️ 开发过程
* 开发过程
```bash
   cargo new rust_tetris && cd rust_tetris
   cargo add macroquad 
``` 
* 核心拆分 
    * 数据结构：二维 [[u8; 10]; 20] 网格
    * 方块：7 种形状 + 4 个旋转状态，常量数组 SHAPES
    * 逻辑：碰撞检测 valid()、消行 clear_lines()、锁定 lock()
    * 渲染：macroquad 的 draw_rectangle 逐格绘制
* 窗口自适应 
```bash
request_new_screen_size(BOARD_W, BOARD_H);
```
让窗口刚好包住棋盘，避免黑边。

* Web 支持 (可选)
```bash
cargo install cargo-web
cargo web start --target wasm32-unknown-unknown
```
---

## 🚀 运行 

```bash
cargo run          # 本地
# 浏览器
cargo web start    # 自动打开 http://localhost:8000
```

--- 

## 🎮 操作 

| 键位      | 功能                 |
| ------- | ------------------ |
| `← / →` | 左右移动               |
| `↓`     | 软降                 |
| `↑`     | 顺时针旋转              |
| `Space` | 暂停 / 继续            |
| `R`     | 重新开始（Game Over 画面） |

---

### ✅ 待办（欢迎 PR） 

1. [] 音效(`macroquad::audio`)
2. [] 关卡加速
3. [] 排行榜持久化

---


## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.