use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");
    loop {
        println!("Please enter your first number:");

        let num1 = read_number();

        println!("Please enter your second number:");

        let num2 = read_number();

        println!("Please enter the operation you would like to perform (+, -, *, /, %, ^, abs, sqrt, log, sin, cos, tan):");

        let operation = read_operation();

        let result = match operation {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            '%' => num1 % num2,
            '^' => num1.powf(num2),
            'a' => num1.abs(),
            's' => num1.sqrt(),
            'l' => num1.log(num2),
            'i' => num1.sin(),
            'o' => num1.cos(),
            't' => num1.tan(),
            _ => {
                println!("Invalid operation");
                continue;
            }
        };

        println!("The result of {} {} {} is {}", num1, operation, num2, result);

        if !continue_calculation() {
            break;
        }
    }
}

fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number")
        }
    }
}

fn read_operation() -> char {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let operation = input.trim().chars().next();

        match operation {
            Some(op) => return op,
            None => println!("Please enter a valid operation")
        }
    }
}

fn continue_calculation() -> bool {
    loop {
        println!("Would you like to perform another calculation? (y/n)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Invalid input")
        }
    }
}
