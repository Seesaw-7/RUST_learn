// fn main() {
//     let _x = 2.0; // f64
//     //unused variable: `x`, if this is intentional, prefix it with an underscore: `_x`
//     let _y: f32 = 3.0; // f32
// }


// fn main() {
//     // addition
//     let _sum = 5 + 10;

//     // subtraction
//     let _difference = 95.5 - 4.3;

//     // multiplication
//     let _product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("{quotient}");
//     let truncated = -5 / 3; // Results in -1
//     println!("{truncated}");

//     // remainder
//     let _remainder = 43 % 5;
// }


// fn main() {
//     let _t = true;

//     let _f: bool = false; // with explicit type annotation
// }


// fn main() {
//     let _c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     println!("{z}");
//     let heart_eyed_cat = '😻';
//     println!("{heart_eyed_cat}");
//     // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, 
//     //which means it can represent a lot more than just ASCII. 
//     //Accented letters; Chinese, Japanese, and Korean characters; 
//     //emoji; and zero-width spaces are all valid char values in Rust. 
// }


// fn main() {
//     let _tup: (i32, f64, u8) = (500, 6.4, 1);
//     let tup2 = (500, 6.4, 1);

//     let (_x, y, _z) = tup2;

//     println!("The value of y is: {y}");

//     let five_hundred = tup2.0;
//     println!("{five_hundred}");

//     let six_point_four = tup2.1;
//     println!("{six_point_four}");
// }
//The tuple without any values has a special name, unit. 
//This value and its corresponding type are both written () and represent an empty value or an empty return type. 
//Expressions implicitly return the unit value if they don’t return any other value.


// fn main() {
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     let b = [3; 5];//let b = [3, 3, 3, 3, 3];

//     let first = a[0];
// }
//However, arrays are more useful when you know the number of elements will not need to change. 
//For example, if you were using the names of the month in a program, you would probably use an array rather than a vector 
//because you know it will always contain 12 elements


use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
