use std::io;

/*
Example shows when ownership applies (stack vs heap)
Showing different approaches to solving our problem (deep copy and Borrowing, Mutable Borrows)

*/

fn greet(subject: String) {
    println!("Hello, {}!", subject.trim());
}

fn concat(subject: &mut String) {
    subject.pop();
    subject.push_str(" Cmiel");
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {

    // heap
    let mut name = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Unable to read from STDIN");

    concat(&mut name);
    greet(name.clone());

    println!("Your name is {} characters long", name.replace(" ", "").len());

    // stack
    let x = 1;
    let y = 2;

    let z = sum(x, y);
    println!("{} + {} = {}", x, y, z);
}
