# deptr

`deptr` is a dependency tracker, written in Rust.

It works for Python Poetry projects, scanning the `pyproject.toml` file for dependencies and then recursively scanning Python source code to find if those dependencies are used.

## Installation

### From source

```bash
git clone {this repo}
cd deptr
cargo install --path .
```

## Usage

```bash
deptr --help
```

## Dev

```bash
cargo run -- --help
cargo run path/to/pyproject
```
