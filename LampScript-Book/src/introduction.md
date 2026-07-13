# Introduction to LampScript

Welcome to **LampScript**! LampScript is a low-level, high-performance, compiled programming language with advanced safety features. It compiles directly into raw x64 Assembly for the Windows platform.

## Key Features

- **No Garbage Collector**: LampScript relies on strict compiler-enforced Ownership rules to manage your memory, providing deterministic performance.
- **Fast and Predictable**: Since it translates to NASM assembly natively, you are as close to the hardware as possible while maintaining safety bounds.
- **C-Style Syntax with Modern Traits**: The syntax is heavily inspired by Rust and C++, favoring explicit design over implicit magic.

## First Program

```rust
fn main() -> void {
    println?("Hello, LampScript World!");
};

{
    main();
};
```
