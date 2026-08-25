# rust-practice

A personal Rust practice repository for learning and experimenting with the Rust programming language.

The repository is organized as a **Cargo workspace**, where each exercise or experiment is its own independent Rust package/crate. This keeps individual topics isolated, easy to run, and easy to revisit later.

## Structure

```text
rust-practice/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── README.md
└── exercises/
    ├── ex01_basics/
    ├── ex02_functions/
    ├── ex03_structs/
    ├── ex04_ownership/
    └── ...
```

Each directory under `exercises/` is an independent Cargo package and can contain a small runnable program focused on a specific Rust concept.

## Creating a New Exercise

From the workspace root:

```powershell
cargo new exercises/ex01_basics --vcs none
```

The workspace automatically includes packages created under `exercises/`.

## Running an Exercise

Run a specific exercise from the workspace root:

```powershell
cargo run -p ex01_basics
```

Alternatively, enter the exercise directory and run it directly:

```powershell
cd exercises/ex01_basics
cargo run
```

## Building the Workspace

Build all workspace packages:

```powershell
cargo build --workspace
```

or:

```powershell
cargo build -w
```

## Running Tests

Run tests for the entire workspace:

```powershell
cargo test --workspace
```

Run tests for a specific exercise:

```powershell
cargo test -p ex01_basics
```

## Purpose

This repository is intentionally kept simple.

The goals are to:

* learn Rust concepts through small runnable examples
* keep each topic isolated in its own crate
* experiment freely without affecting other exercises
* build a collection of examples that can be revisited later
* become familiar with Cargo, crates, modules, workspaces, and the Rust ecosystem
* gradually progress from basic language concepts toward larger Rust programs

The exercises are primarily learning material rather than production-ready implementations.
