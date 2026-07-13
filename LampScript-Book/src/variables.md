# Variables and Mutability

In LampScript, variables are **immutable by default**. This means once you bind a value to a name, you cannot change it. This leads to safer and more predictable code.

## Immutable Variables

To declare a variable, use the `let` keyword:

```rust
let x: u32 = 10;
// x = 20; // This will cause a Compilation Error!
```

## Mutable Variables

If you need a variable to be mutable, you must explicitly declare it using the `mod` keyword.

```rust
let y: mod f64 = 2.3;
y = 5.5; // Perfectly valid
```

The type annotation (`: mod type`) clearly indicates to the reader and the compiler that this variable is subject to modification.
