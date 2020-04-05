fn main() {
    println!("Hello, world!2");

    let a = [1,2,3,4,5];
    let index = 4;
    let element = a[index];
    println!("element: {}", element);

    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &mut s;
    // let r2 = &mut s; //error
    println!("{}", r1);
    // println!("{}, {}", r1, r2); //error
    {
        let r3 = &mut s;

        println!("{}", r3);
        // println!("{}, {}", r1, r3); //error
    }

    println!("{}",s);

    let r4 = &s;
    let r5 = &s;
    // let r6 = &mut s; //error
     println!("{}, {}", r4, r5); //error

    // let r7 = r1; // error
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
