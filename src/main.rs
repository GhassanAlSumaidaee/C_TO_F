use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        println!("Choose an option:");
        println!("1. Convert from Fahrenheit to Celsius");
        println!("2. Convert from Celsius to Fahrenheit");
        println!("3. Quit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter temperature in Fahrenheit:");

                let mut fahrenheit = String::new();

                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Failed to read line");

                let fahrenheit: f64 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                println!("Temperature in Celsius: {:.2}", celsius);
            }
            2 => {
                println!("Enter temperature in Celsius:");

                let mut celsius = String::new();

                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to read line");

                let celsius: f64 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
                println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
            }
            3 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Please choose again."),
        }
    }
}
