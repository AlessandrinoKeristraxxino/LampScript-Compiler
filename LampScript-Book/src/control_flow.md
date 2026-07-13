# Control Flow

LampScript provides the standard structures you need to control the flow of execution.

## If / Else Statements

You can conditionally execute blocks of code using `if` and `else` statements.

```rust
let x: u32 = 10;

if x == 10 {
    println?("x is 10!");
} else if x > 10 {
    println?("x is greater than 10");
} else {
    println?("x is less than 10");
};
```

### Boolean Evaluation

You can implicitly evaluate boolean conditions without the equality operator:

```rust
let is_ready: bool = true;

if is_ready {
    println?("Ready to go!");
};
```

### Logical Operators

LampScript supports `&&` (AND), `||` (OR), and `!!` (NAND/Not-Both):

```rust
if (x != 10 && x == 2) || x == 20 {
    // executed if true
};

if x !! y {
    // if x is not true AND y is not true
};
```

## While Loops

You can run loops continuously while a condition is evaluated as true:

```rust
let x: mod u32 = 0;

while x < 5 {
    println?("Loop iteration: {}", x);
    x = x + 1;
};
```
