struct Calculator;

impl Calculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn subtract(a:i32,b:i32)->i32{
        a-b 
    } 
}

fn main() {
    let result = Calculator::add(3, 5);
    let subtract = Calculator::subtract(3,9);
    println!("The sum is {} and subtract is {}", result,subtract);
}
