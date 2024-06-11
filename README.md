# deptr

`deptr` is a dependency tracker, written in Rust.

It works for Python Poetry projects, scanning the `pyproject.toml` manifest file for dependencies and then recursively scanning the project's Python source code to find if the dependencies are in the manifest..

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
