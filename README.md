# LampScript-Compiler
The official compiler for the LampScript Lang (LAS).

## What is LampScript?
LampScript is a lightweight programming language with a simple syntax for basic declarations and output operations.

```rs
let x = 10; // supports u64

print?();
print?(x);
```

## Features
- Simple variable declarations with `let`
- Basic output via `print?()`
- Compilation to assembly and executable output

## Installation
On Windows, from the project root, run:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

This script builds the project and installs the `las` command globally in your user PATH.

## Usage
After installation, you can compile and run a source file with:

```powershell
las run program.las
```

If you do not pass a file, the compiler will default to `program.las`.

## Development
To build locally:

```powershell
cargo build --release
```

To run the tests:

```powershell
cargo test
```

