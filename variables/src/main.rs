// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// }


// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }
//Shadowing is different from marking a variable as mut because we’ll get a compile-time error 
//if we accidentally try to reassign to this variable without using the let keyword. 
//By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.


fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
// Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num