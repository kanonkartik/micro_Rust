//fhrenchiat to celcius
use std::io;

fn main() {
    loop {
        println!("For converting F to C, type F");
        println!("For converting C to F, type C");
        println!("To exit, type Q");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_uppercase();

        match input.as_str() {
            "F" => convert_fahrenheit_to_celsius(),
            "C" => convert_celsius_to_fahrenheit(),
            "Q" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid input. Please type F, C, or Q."),
        }
    }
}

fn convert_fahrenheit_to_celsius() {
    loop {
        println!("Please input Fahrenheit (or type Q to go back):");

        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit = fahrenheit.trim().to_uppercase();
        if fahrenheit == "Q" {
            break;
        }

        let fahrenheit: f32 = match fahrenheit.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        let celsius: f32 = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("You entered Fahrenheit: {}", fahrenheit);
        println!("The converted Fahrenheit to Celsius: {:.2}", celsius);
        break;
    }
}

fn convert_celsius_to_fahrenheit() {
    loop {
        println!("Please input Celsius (or type Q to go back):");

        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius = celsius.trim().to_uppercase();
        if celsius == "Q" {
            break;
        }

        let celsius: f32 = match celsius.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        let fahrenheit: f32 = (celsius * 9.0 / 5.0) + 32.0;
        println!("You entered Celsius: {}", celsius);
        println!("The converted Celsius to Fahrenheit: {:.2}", fahrenheit);
        break;
    }
}
