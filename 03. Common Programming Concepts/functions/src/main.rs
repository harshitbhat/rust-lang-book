fn main() {
    another_function();
    another_function_one(5);
    print_labeled_measurement(42, 'h');
    println!("The value of x is {}", five());
    println!("The value of x is {}", plus_one(five()));
}

fn another_function() {
    println!("Another function.")
}

fn another_function_one(x: i32) {
    println!("The value of x is : {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value} {unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
