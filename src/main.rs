use std::io;

fn fibn(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibn(n - 1) + fibn(n - 2);
}

fn main() {
    let mut number = String::new();

    println!("Enter fib sequence: ");
    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("Please enter a number!");

    println!("The {}'th fibinachi number is {}", number, fibn(number));
}
