
// Rust has two main string types:

// 1. String - The dynamic/growable owned string type
let mut s = String::from("Hello");  // Create from literal
s.push_str(", world!");            // Can modify since it's mutable


// 2. &str - The string slice type (immutable view into string data)
let s = "Hello world";  // String literal - type is &'static str
let slice = &s[0..5];   // Take a slice - type is &str


// Key differences between String and &str:

// String is heap-allocated, growable, and owned
// &str is a borrowed view into string data, fixed-size, and immutable
// String can be converted to &str via borrowing (&)
// String owns its data while &str borrows it


let mut s1 = String::new();
s1="hello".to_string()

