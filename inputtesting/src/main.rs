use text_io::{read, try_read};

struct Person { // Struct to hold user data
    name: String,
    age: i32,
    fav_color: String,
}

impl Person { // Implement methods for the struct
    fn print_greeting(&self) { // Method to print a greeting
        println!("Hello, {}! You are {} years old and your favorite color is {}.", self.name, self.age, self.fav_color);
    }
}

fn main() {
    // Prompt the user for their information 
    println!("Enter your name:");
    let user_name: String = read!(); // Reading user input using text_io
    println!("Enter your age:");
    // Use try_read to handle invalid input
    let mut i: Result<i32, text_io::Error> = try_read!(); // i is a Result type containing either an i32 or an Error
    // If invalid, ask again until valid input is given
    while i.is_err() {
        println!("Invalid input. Please enter a number:");
        i = try_read!();
    }
    let user_age: String = i.unwrap().to_string();
    println!("Enter your favorite color:");
    let user_fav_color: String = read!();
    // Create a new Person with the user's data
    let user = Person {
        name: user_name,
        age: user_age.parse().unwrap(),
        fav_color: user_fav_color,
    };
    user.print_greeting(); // Call the print_greeting method on the user
}
