struct Math {
    first_number: i32,
    second_number: i32,
}

impl Math {
    fn new(first_number: i32, second_number: i32) -> Self {
        Self {
            first_number:first_number,
            second_number:second_number,
        }
    }

    fn sum(&self)->i32{
        self.first_number + self.second_number
    }
}

fn main() {
    let math_obj = Math::new(5, 9);
    let sum = math_obj.sum();
    println!("Math Object: first_number = {}, second_number = {}", math_obj.first_number, math_obj.second_number);
}
