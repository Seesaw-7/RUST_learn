use std::io;

fn main() {
    println!("Temperature conversion");

    let cv_type = loop {      
        println!("Plese input a single digit 1 or 2 to select the conversion");
        println!("1. From Fahrenheit to Celsius.");
        println!("2. From Celsius to Fahrenheit");
    
        let mut cv_type = String::new();
        
        io::stdin()
            .read_line(&mut cv_type)
            .expect("Failed to read line");
    
        let cv_type: u8 = match cv_type.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        
        if cv_type == 1 || cv_type == 2{
            break cv_type;
        }

        println!("Wrong input");
    };

    if cv_type == 1 {
        println!("Please enter the temperature in Fahrenheit");

        let mut f = String::new();

        io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");

        let f: f32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        let c = (f - 32.0) * 0.5556;

        println!("Result = {} °C", c);
    } else if cv_type == 2 {
        println!("Please enter the temperature in Celsius");

        let mut c= String::new();

        io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");

        let c: f32 = match c.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };

        let f = c * 1.8 + 32.0;

        println!("Result = {} °F", f);       
    }
}
