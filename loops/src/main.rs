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
}

