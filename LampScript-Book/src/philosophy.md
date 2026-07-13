# The Philosophy of LampScript

LampScript was not born out of a desire to simply add another language to the ecosystem. It was forged with a very specific vision: **To bring the absolute control and raw performance of Assembly programming to a safe, modern, and ergonomic syntax.**

The core tenets of LampScript's design philosophy guide every decision made in its compiler and syntax.

## 1. Predictability Over Magic

In modern programming, languages often hide the complexities of the underlying hardware behind massive Virtual Machines (VMs) or heavy runtimes. While this makes development easier, it introduces "magic" that can lead to unpredictable latency spikes, bloated binaries, and a lack of understanding of what the machine is *actually* doing.

LampScript rejects this approach.

- **Direct Assembly Output**: The compiler translates your code directly into raw, human-readable NASM (Netwide Assembler) x64 Assembly. There is no intermediate VM.
- **Zero Runtime Overhead**: LampScript does not ship with a bloated runtime. Your program starts instantly, executes exactly what you told it to execute, and exits. What you write is what the CPU executes.

## 2. Deterministic Memory Control

Garbage Collectors (GCs) are brilliant inventions, but they come at the cost of deterministic performance. You cannot predict exactly when a GC will pause your program to clean up memory. For systems programming, game engines, and high-performance computing, this is unacceptable.

- **Zero Garbage Collector**: LampScript refuses to use a GC. 
- **Compile-Time RAII**: Instead of a GC, the *Compiler* is the memory manager. By strictly tracking the ownership of variables through scopes, the compiler knows exactly when a variable is no longer needed and injects `HeapFree` calls directly into the Assembly at compile-time. You get the safety of a GC with the absolute performance of manual `C` memory management.

## 3. Explicit Safety Through Ownership

Bugs related to memory manipulation—like "Double Free" and "Use After Free"—are among the most catastrophic and difficult to debug. LampScript believes that the language should prevent you from making these mistakes before your program even runs.

- **Ownership as a First-Class Citizen**: By enforcing strict rules about who "owns" a piece of memory, LampScript prevents multiple variables from claiming the same heap allocation.
- **Move Semantics**: When complex types (like `string`) are assigned to a new variable, ownership is *Moved*, not copied. The compiler mathematically guarantees that memory is freed exactly once. 
- **Borrowing (`&`)**: If you need to share access without transferring ownership, LampScript provides explicit borrowing. The `&` operator ensures you are fully aware when you are handling a pointer versus an owned value.

## 4. Immutability by Default

State mutation is the root of many logical bugs and concurrency issues. If data can change unexpectedly, reasoning about your program becomes exponentially harder.

- **Explicit Mutation**: In LampScript, variables are immutable by default. You cannot accidentally overwrite a value. If you need a variable to change, you must explicitly declare it with the `mod` keyword. 
- This deliberate choice forces the developer to declare their *intent* to mutate, making the code self-documenting and intrinsically safer.

## 5. Ergonomics Without Compromise

Writing raw Assembly or `C` is powerful, but it is also tedious and error-prone. LampScript aims to provide the elegant, modern syntax of languages like Rust or TypeScript while maintaining the raw horsepower of low-level languages.

We believe that **System-Level Control should not require archaic syntax**. You get the beautiful structure of `{ ... }` blocks, straightforward type annotations (`u32`, `f64`), and clear control flow, all while writing instructions that map almost 1:1 with CPU opcodes.

## Summary

The LampScript Philosophy is simple: **Empower the developer.**

Give them the speed of C. Give them the memory safety of Rust. Give them the readability of modern scripts. Hide nothing from them, but protect them from themselves. That is LampScript.
