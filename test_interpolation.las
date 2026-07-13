fn test_interpolation() -> void {
    let x: u32 = 42;
    let y: string = alloc "world";

    println?("Interpolation anonymous: {}, {}", x, &y);
    println?("Interpolation named: x is {x} and string is {y}");
};

{
    test_interpolation();
};
