fn main() {
    println!("Hello, world!");
    another_function();
    parameter_function(2,3);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("x: {}\ny: {}", x, y);

    println!("plus one: {}", plus_one(5));
}

fn another_function(){
    println!("another function");
}

fn parameter_function(x: i32, y: i32) {
    println!("sum: {}", x + y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
