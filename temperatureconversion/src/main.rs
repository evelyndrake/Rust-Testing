use text_io::try_read;

fn fahrenheit_to_celsius(f: f64) -> f64 { // Function to convert Fahrenheit to Celsius
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 { // Function to convert Celsius to Fahrenheit
    c * 9.0 / 5.0 + 32.0
}

fn get_valid_float() -> f64 { // Function to get a valid floating-point number from the user
    let mut f: Result<f64, text_io::Error> = try_read!(); // Read user input as a Result type
    while f.is_err() { // If the input is invalid, ask again until valid input is given
        println!("Invalid input. Please enter a number:");
        f = try_read!();
    }
    f.unwrap() // Return the valid floating-point number
}

fn get_valid_int() -> i32 { // Function to get a valid integer from the user (for selection, should be 1 or 2)
    let valid_int: i32; // Declare a variable to hold the valid integer
    loop { // Loop until a valid integer is entered
        let i = try_read!(); // Read user input as a Result type
        match i { // Match over the Result type
            Ok(val) if val == 1 || val == 2 => { // If the input is 1 or 2, assign it to valid_int and break the loop
                valid_int = val;
                break;
            }, // Otherwise, print an error message and continue the loop
            _ => println!("Invalid input. Please enter 1 or 2:"),
        }
    }
    valid_int // Return the valid integer
}

fn main() {
    // Prompt user to select a conversion
    println!("Select a conversion:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    let conversion: i32 = get_valid_int(); // Get user input
    match conversion { // Match over the user's selection
        1 => {
            println!("Enter a temperature in Fahrenheit:");
            let f: f64 = get_valid_float();
            // I added {:.2} to format the output to two decimal places
            println!("{:.2}°F is equal to {:.2}°C", f, fahrenheit_to_celsius(f));
        }
        2 => {
            println!("Enter a temperature in Celsius:");
            let c: f64 = get_valid_float();
            println!("{:.2}°C is equal to {:.2}°F", c, celsius_to_fahrenheit(c));
        }
        _ => println!("Invalid selection"),
    }
}
