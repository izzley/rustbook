// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }
use std::io;

fn main() {
    // addition
    let sum = 5 +10;
    println!("The sum of 5 + 10: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference 95.5 - 4.3: {}", difference);

    // multiplication
    let product = 4*30;
    println!("The product of 4*3: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient of 56.7 / 32.2: {}", quotient);


    let floored = 2 / 3; // results in 0
    println!("The floored of 2 over 3: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The remainder of 43 / 3: {}", remainder);

    let t: bool = true;
    println!("The true value t: {}", t);
    let f = false;
    println!("The false value f: {}", f);

    //tuples are compound variable and can be destructured

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("first tuple desctructured: {}", five_hundred);

    let six_point_four = x.1;
    println!("second tuple desctructured: {}", six_point_four);

    let one = x.2;
    println!("third tuple desctructured: {}", one);

    // arrays are fixed in length. Vectors are not
    let a: [i32; 5] = [1,2,3,4,5];

    let first = a[0];
    println!("first element of a: {}", first);

    let second = a[1];
    println!("second element of a: {}", second);
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}