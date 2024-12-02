use std::io;

fn input(printable: &str) -> String {
    println!("{}", printable);

    let mut input_data = String::new(); // Create a mutable String variable to hold the input
    io::stdin() // Read input from stdin
        .read_line(&mut input_data) // Read the line into the variable
        .expect("Failed to read line"); // Handle any error

    input_data.trim().to_string() // Return the input without the trailing newline
}

fn main() {
    let input_data = input("Enter any number:");
    let apple = "hello";
    println!("You entered: {} {}", input_data,apple); // Output the data entered by the user
}
