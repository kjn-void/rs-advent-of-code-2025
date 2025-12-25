# Advent of Code 2025 (Rust)

This repository contains a clean, extensible **Rust framework** for solving **Advent of Code 2025**, built around a simple trait-based architecture and fast benchmarking.

Each day’s solution implements a shared trait and is registered in a central registry, making it easy to run, benchmark, and extend.

---

## 🧩 Core Interface

All days implement the following trait:

```rust
pub trait Solution {
    fn set_input(&mut self, lines: &[String]);
    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;
}
```

Each day registers itself via `days::make_solver(day)`.

---

## 📦 Project Structure

```
rs-advent-of-code-2025/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
│
├── src/
│   ├── main.rs            # CLI entry point (clap-based)
│   ├── lib.rs             # library root
│   │
│   ├── aocnet.rs          # AoC input downloader (session cookie)
│   │
│   └── days/
│       ├── mod.rs         # Solution trait + registry
│       ├── day01.rs
│       ├── day02.rs
│       ├── ...
│       └── day12.rs
│
├── input/                 # cached inputs (auto-created)
│   ├── day01.txt
│   ├── ...
│   └── day12.txt
│
└── benches/
    └── bench_days.rs      # Criterion benchmarks
```

---

## 🚀 Running Solutions

Run a single day:

```bash
cargo run -- --day 9
```

Run a single part only:

```bash
cargo run -- --day 9 --part 1
```

Run and force online fetch:

```bash
cargo run -- --day 9 --fetch
```

If `--part` is omitted, **both parts are executed**.

---

## 🌐 Automatic Input Download (adventofcode.com)

This framework supports **automatic input downloading** using your personal Advent of Code session cookie.

AoC does **not** provide OAuth or an API.  
Instead, authentication is done via a cookie named:

```
session=YOUR_SESSION_TOKEN
```

The Rust implementation lives in:

```
src/aocnet.rs
```

---

## 🔑 How to Retrieve Your Session Token

1. Log in at https://adventofcode.com  
2. Open browser Developer Tools  
   - Safari: ⌥ Option + ⌘ Command + I  
   - Chrome: F12 → Application tab  
   - Firefox: F12 → Storage tab  
3. Cookies → `https://adventofcode.com`  
4. Copy the value of the cookie named `session`

⚠️ **This token is private. Do NOT commit it.**

---

## 🧷 Enabling Automatic Download

Set environment variables:

```bash
export AOC_SESSION="your-session-token"
export AOC_ONLINE=1
```

Now when you run:

```bash
cargo run -- --day 9
```

The program will:

1. Fetch  
   `https://adventofcode.com/2025/day/9/input`
2. Save it to  
   `input/day09.txt`
3. Use the cached file for future runs

If fetching fails, it **automatically falls back** to the local file.

---

## ⏱️ Benchmarks (Criterion)

Benchmarks are implemented using **Criterion.rs**, mirroring the Go benchmark structure.

### Run all benchmarks

```bash
cargo bench
```

### Run only “full pipeline” benchmarks

```bash
cargo bench full
```

(or more strictly)

```bash
cargo bench day.*_full
```

Each day benchmarks:

- `set_input`
- `part1`
- `part2`
- `full` (set_input + part1 + part2)

Bench definitions live in:

```
benches/bench_days.rs
```

---

## 📊 Benchmark Summary — Apple M4 (darwin/arm64)

| Day | SetInput (µs) | Part1 (µs) | Part2 (µs) | Full (µs) |
| --- | ------------- | ---------- | ---------- | --------- |
| 01  |               |            |            | 589       |
| 02  |               |            |            | 15        |
| 03  |               |            |            | 56        |
| 04  |               |            |            | 256       |
| 05  |               |            |            | 18        |
| 06  |               |            |            | 183       |
| 07  |               |            |            | 28        |
| 08  |               |            |            | 11_000    |
| 09  |               |            |            | 33_000    |
| 10  |               |            |            | 70_000    |
| 11  |               |            |            | 268       |
| 12  |               |            |            | 206       |

---

## ✅ Design Goals

- Zero global state
- Explicit input ownership
- Fast iteration & benchmarking
- Identical semantics to Go version
- Easy extension to Day 25+

---
