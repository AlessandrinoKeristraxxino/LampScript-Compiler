struct Vector {
    x: u32,
    y: u32,
}

fn main() -> void {
    let v: Vector = Vector {
        x: 10,
        y: 20,
    };
    
    println?("Vector created! X is {}, Y is {}", v.x, v.y);
};

{
    main();
};
