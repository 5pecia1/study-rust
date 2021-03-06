fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number -1;
    }

    println!("Launch!");

    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("element: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("Launch!");

    let mut s = String::new();
    s.push_str("def");

}
