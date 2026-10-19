use colored::Colorize;
use std::io;

fn main() {
    loop {
        println!("Enter the temperature scale ('c' for Celsius and 'f' for Farenheit");
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read input!");
        let scale: char = match choice.trim() {
            "c" => 'c',
            "C" => 'c',
            "f" => 'f',
            "F" => 'f',
            _ => {
                println!("Invalid scale chosen! Please try again.");
                continue;
            }
        };

        println!("Enter the temperature:");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the input!");

        let temp: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a number!".red());
                continue;
            }
        };

        let result = match scale {
            'c' => c_to_f(temp),
            'f' => f_to_c(temp),
            _ => 0.0,
        };

        println!("The converted temperature is: {result}");
    }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn c_to_f(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}
