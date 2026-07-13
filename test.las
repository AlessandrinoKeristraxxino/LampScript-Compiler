let global_val: mod u32 = 100;

fn modify_global() -> void {
    global_val = global_val + 50;
    println?("Global modified inside function: {}", global_val);
};

fn sum(x: mod u32, y: u32) -> u32 {
    x = x + y;
    return x;
};

let a: u32 = 10;
let b: u32 = 20;

let result: u32 = sum(a, b);
println?("Result of sum(10, 20): {}", result);

modify_global();
println?("Global value after function call: {}", global_val);

if result == 30 {
    println?("Logic works!");
};

let i: mod u32 = 0;
while i < 2 {
    println?("Loop iteration {}", i);
    i = i + 1;
};

