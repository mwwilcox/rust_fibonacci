use std::io;
use std::io::Write;

fn fibn(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibn(n - 1) + fibn(n - 2);
}

fn match_fib() {
    print!("Enter fib sequence: ");
    io::stdout().flush().unwrap();
    
    let mut number = String::new();
    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("Please enter a number!");

    println!("The {}'th fibinachi number is {}", number, fibn(number));
}

// fn fahrenheit_to_celsius(temp: i32) -> i32 {
//     (temp - 32) * (5/9)
// }

fn celsius_to_fahrenheit(temp:i32) -> f32 {
    (temp as f32 * 1.8) + 32.0
}

fn match_temp() {
    let choices: Vec<&str> = vec!["Celsius to Fahrenheit", "Fahrenheit to Celsius"];
    for (index, item) in choices.iter().enumerate() {
        println!("{}: {}", index, item);
    }
    print!("Choose operation: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid choice");

    let test = &choice.trim().parse::<i32>();
    match test {
        Ok(_ok) => {
            let index: usize = choice.trim().parse().expect("Expecting a number");
            if index < choices.len() {
                if index == 0 {
                    print!("Enter a temperature in Celsius: ");
                    io::stdout().flush().unwrap();

                    let mut temp = String::new();
                    io::stdin().read_line(&mut temp).expect("you must enter a number!");
                    let temp: i32 = temp.trim().parse().expect("Expecting a number!");
                    let res = celsius_to_fahrenheit(temp);
                    println!("{}C is {}F", temp, res);
                }
            }
        }
        Err(e) => {
            println!("Error: {e}")
        }
    }
}

fn main() {
    let choices: Vec<&str> = vec!["fibonacci number", "temperature conversion"];

    println!("Chapter 3 roundup - choose a function from the list!");
    loop {
        for (index, item) in choices.iter().enumerate() {
            println!("{}: {}", index, item);
        }
        println!("---------------------------\n");
        print!("Function: ");
        io::stdout().flush().unwrap();

        let mut menu_choice = String::new();
        io::stdin().read_line(&mut menu_choice).expect("Invalid Choice!");

        // Here, the menu choice could be either a number represented as a string
        // or a string equal to one of the menu choices (if the user actually typed out the choice)
        // So, check for each

        // Try converting menu_choice to a int32, if it fails see if the string is in the list
        let test = &menu_choice.trim().parse::<i32>();
        match test {
            Ok(_ok) => {
                let index: usize = menu_choice.trim().parse().expect("Expecting a number");
                if index < choices.len() {
                    if index == 0 {
                        match_fib();
                    } else if index == 1 {
                        match_temp();
                    }
                } else {
                    println!("You must choose a number within the list or I will just keep doing this!")
                }
            },
            Err(e) => println!("User didn't enter a number {}", e),
        }
    }
}
