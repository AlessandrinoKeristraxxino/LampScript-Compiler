# Memory Management & Ownership

LampScript guarantees memory safety **without** using a Garbage Collector. It accomplishes this through a strict **Ownership** and **Borrowing** architecture directly integrated into the compiler.

## Heap Allocation (`alloc`)

You can dynamically allocate memory directly on the OS heap (via Windows `HeapAlloc`) using the `alloc` keyword:

```rust
let my_data: string = alloc "dynamic data";
```

## RAII (Automatic Drop)

Whenever a variable is instantiated inside a scope `{ ... }`, the compiler becomes its **Owner**. When that scope naturally ends, the compiler injects an automatic `HeapFree` instruction directly into the compiled assembly to destroy the object and reclaim memory. 

You never write `free()` or `delete`.

## Move Semantics

To prevent "Double Free" attacks, the compiler ensures there is only ever ONE owner of a specific piece of heap memory.

Primitive types (`u32`, `bool`) are automatically **Copied**.
Complex types (`string`) are automatically **Moved**.

```rust
let a: string = alloc "hello";
let b: string = a; // Ownership transferred to 'b'
```

Once `a` is moved to `b`:
1. `b` becomes the sole owner of the memory.
2. `a` is flagged internally by the compiler as `Moved`.
3. At the end of the block, the compiler *skips* injecting `HeapFree` for `a` and only frees `b`. This prevents a double free!
4. Any attempt to read `a` afterward causes a compiler **Security Error**.

## Borrowing

If you do not wish to move ownership into a function or a macro, you can **Borrow** the variable using the Ampersand `&` operator:

```rust
let text: string = alloc "hello";
println?("The value is: {}", &text); 
// text is NOT moved, and is still valid!
```
