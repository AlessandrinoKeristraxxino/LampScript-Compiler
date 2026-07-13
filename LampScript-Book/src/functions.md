# Functions

Functions are defined using the `fn` keyword. 

## Declaring Functions

Every function must declare its parameter types and its return type (which can be `void`).

```rust
fn sum(x: i32, y: i32) -> i32 {
    return x + y;
};
```

*Note: You must conclude function declarations with a semicolon `;` after the closing brace!*

## Mutable Parameters

By default, arguments passed into functions are immutable inside the function body. If you wish to mutate an argument internally, prefix the type with `mod`:

```rust
fn increment(x: mod u32) -> u32 {
    x = x + 1;
    return x;
};
```

## Borrowing Parameters

If you are passing a complex heap-allocated type (like an `alloc string`) into a function, passing it by value will transfer ownership to the function, and it will be destroyed when the function ends. 

To avoid this, you can pass parameters by **Reference** using the Ampersand `&` borrow operator. In LampScript, the `&` must be placed to the *left* of the parameter name:

```rust
fn print_length(&text: string) -> void {
    // The function borrows the string, it does NOT own it
    println?("String is: {}", text);
};

let my_str: string = alloc "hello";
print_length(&my_str); // Passed by reference
// my_str is still valid here!
```
