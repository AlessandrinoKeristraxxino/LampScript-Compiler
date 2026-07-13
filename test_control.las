fn test_control() -> void {
    let x: u32 = 10;
    let y: u32 = 2;
    
    if x {
        println?("Implicit bool evaluation works! (x is not 0)");
    };

    if x == 10 && y == 2 {
        println?("Logical AND works!");
    };

    if x == 1 || y == 1 {
        println?("This should not print.");
    }; else if (x != 10 && y == 2) || x == 10 {
        println?("Else if and parenthesis grouping works!");
    };

    if x !! x {
        println?("This should not print, x AND x is true, so NOT BOTH is false.");
    }; else {
        println?("NAND logic works!");
    };
};

{
    test_control();
};
