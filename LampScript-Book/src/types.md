# Data Types

LampScript is statically typed. Every variable must have a known type at compile time.

## Primitive Types

The language natively supports a wide variety of signed/unsigned integers and floating-point numbers:

- **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`
- **Signed Integers**: `i8`, `i16`, `i32`, `i64`
- **Floating Points**: `f8`, `f16`, `f32`, `f64`
- **Booleans**: `bool` (`true` or `false`)
- **Characters**: `char`

*Note: All of these primitive types are treated as **Copy** types in LampScript's ownership model.*

## Complex Types

- **Strings**: `string`

Strings are treated as **Move** types by the compiler. Due to their dynamic nature, assigning a string to a new variable transfers ownership.

```rust
let text: string = "hello";
let other_text: string = text; 
// text is now moved, accessing it will cause a compiler error
```
