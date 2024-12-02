

struct Book {
    title: String,
    author: String,
}

fn main() {
    // Convert the &str to String
    let mut title_1: String = String::from("hello"); // or use title_1 = "hello".to_string();
    println!("{}", title_1);  // Add a semicolon at the end of println!

    let book1 = Book {
        title: title_1,
        author: String::from("Md"),
    };
    
    // You can now use `book1` to print its details or work with it.
    println!("Book Title: {}, Author: {}", book1.title, book1.author);
}
