# ffi-python

This project is created for learning purposes to execute functions implemented in Rust from Python3.

## Project Overview

This is a sample project to learn how to use Rust's FFI (Foreign Function Interface) to call functions implemented in Rust from Python.

## Prerequisites

- Python 3.13 or higher
- Rust
- Maturin

## Setup

Run the following command to set up the project:

```bash
uv sync
```

## Build Instructions

To build the Rust code as a Python extension module, run the following command:

```bash
uv run maturin develop
```

This will install the extension module in your local environment.

## How to Run

Run `main.py` to see an example of calling a Rust function from Python:

```bash
uv run main.py
```

The output will be as follows:

```
3
```

## Directory Structure

```
ffi-python/
├── .gitignore          # Git ignore file
├── .python-version     # Python version specification
├── Cargo.lock          # Rust dependency lock file
├── Cargo.toml          # Rust project configuration file
├── main.py             # Python entry point
├── pyproject.toml      # Python project configuration file
├── README.md           # Project description file
├── src/                # Rust and Python source code
│   ├── lib.rs          # Rust library code
│   └── example/        # Python module
│       ├── __init__.py # Python module initialization file
│       ├── _core.pyi   # Python type definition file
│       └── _core.cpython-*.so # Python extension module built from Rust
└── target/             # Rust build artifacts
```
