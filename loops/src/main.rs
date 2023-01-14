// fn main() {
//     loop {
//         println!("again!");
//     }
// }

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };//After the loop, we use a semicolon to end the statement that assigns the value to result.

    println!("The result is {result}");


    //If you have loops within loops, break and continue apply to the innermost loop at that point. 
    //You can optionally specify a loop label on a loop that you can then use with break or continue 
    //to specify that those keywords apply to the labeled loop instead of the innermost loop. 
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;

        
    }
    println!("End count = {count}");
    

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    //This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    //This while is error prone. 
    // It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.


    //Looping through each element of a collection using a for loop
    //Collection!!!
    for element in a {
        println!("the value is: {element}");
    }


    //The safety and conciseness of for loops make them the most commonly used loop construct in Rust
    //Even in situations in which you want to run some code a certain number of times, as in the countdown example, 
    //most Rustaceans would use a for loop. 
    //The way to do that would be to use a Range, provided by the standard library, 
    //which generates all numbers in sequence starting from one number and ending before another number.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

