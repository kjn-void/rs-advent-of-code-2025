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
├── problems.yaml          # brief puzzle descriptions for verbose mode
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

Run with a brief puzzle description before solving:

```bash
cargo run -- --day 9 --verbose
```

The short form is also available:

```bash
cargo run -- --day 9 -v
```

Verbose mode reads the embedded `problems.yaml` metadata and prints the selected day’s title and one-line description before running the solver.

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

## 📊 Benchmark Summary — Mac Mini, Apple M4 (darwin/arm64)

Measured with Rust 1.97.0 and Criterion's default 100-sample configuration.

| Day | Full pipeline (µs) |
| --- | ------------------ |
| 01  | 44.52              |
| 02  | 17.01              |
| 03  | 61.35              |
| 04  | 211.59             |
| 05  | 20.02              |
| 06  | 19.04              |
| 07  | 29.70              |
| 08  | 3_134.70           |
| 09  | 473.32             |
| 10  | 2_631.70           |
| 11  | 101.05             |
| 12  | 214.11             |

---

## ✅ Design Goals

- Zero global state
- Explicit input ownership
- Fast iteration & benchmarking
- Identical semantics to Go version
