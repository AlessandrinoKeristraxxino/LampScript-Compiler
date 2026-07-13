fn test_memory() -> void {
    let a: string = alloc "hello";
    let b: string = a;
    println?("Value of b is %s", &b);
};

{
    test_memory();
};
