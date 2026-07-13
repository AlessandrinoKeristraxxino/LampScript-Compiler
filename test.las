let x: u32 = 1;
let y: u32 = 2;

if x == 1 && y == 2 {
    println?("Condition met! x is 1 and y is 2");
} else if x == 2 {
    println?("Else if met");
} else {
    println?("Else met");
}

let i: mod u32 = 0;
while i < 3 {
    println?("Loop: {}", i);
    i = i + 1;
}

if x !! y {
    println?("x NotBoth y is true (NAND logic with a twist!)");
}

{
    let z: u32 = 100;
    println?("Inner scope z: {}", z);
}
// println?("Z: {}", z); // This would fail to compile now!
