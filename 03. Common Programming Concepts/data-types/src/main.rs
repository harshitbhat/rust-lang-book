use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {guess}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let (x, y, z) = tup;
    println!("{x} - {y} - {z}");

    let a = [1, 2, 3, 4, 5]; // compiled type :[i32, 5]
    println!("{:?}", a);

    let a = [3; 5];
    println!("{:?}", a);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index) // returns -> Result<usize, Error>
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse() // returns -> Result<usize, ParseIntError>
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // If index > array length - 1
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:30:19
}
