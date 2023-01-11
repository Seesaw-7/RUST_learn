// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }


// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }


// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };//the block {} evaluates to 4 (no semicolumns)

//     println!("The value of y is: {y}");
// }
//Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. 
//Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11. 
//Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6. 
//Calling a function is an expression. Calling a macro is an expression. 
//A new scope block created with curly brackets is an expression


// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }


fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

