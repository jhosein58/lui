# LUI — Light User Interface

LUI is a lightweight, CPU-based GUI library written in Rust.  
It provides fast, responsive interfaces without relying on GPU or shaders — perfect for prototyping, desktop tools, or embedded visuals.

## ✨ Features

- 🧠 **CPU-based rendering** — works everywhere, no graphics driver required
- ⚡ **Fast & reactive** — optimized for low-latency updates
- 🎯 **Easy to use** — minimal setup, clean API
- 🖼️ **Resizable window support** — dynamic layouts from day one

---

## 🚀 Getting Started

Here's a minimal example:

```rust
fn main() {
    let mut sc = Screen::new("test", 200, 200);

    while sc.is_open() {
        let (w, h) = sc.size();
        let buf = vec![0u32; w * h]; // fill with pixels
        sc.update(&buf);
    }
}
