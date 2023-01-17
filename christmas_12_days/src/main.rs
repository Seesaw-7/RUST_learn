fn main() {
    println!("On the first day of christmas,");
    println!("My true love sent to me");
    println!("A partridge in a pear tree");
    println!("");

    for day in 2..13 {
        //can be better solved by macro or enum.. but 我不會呀
        let dayth = if day == 2 {
            "second"
        } else if day == 3 {
            "third"
        } else if day == 4 {
            "fourth"
        } else if day == 5 {
            "fifth"
        } else if day == 6 {
            "sixth"
        } else if day == 7 {
            "seventh"
        } else if day == 8 {
            "eighth"
        } else if day == 9 {
            "ninth"
        } else if day == 10 {
            "tenth"
        } else if day == 11 {
            "eleventh"
        } else {
            "twelfth"
        };
        println!("On the {} day of christmas", dayth);
        println!("My true love sent to me");
        gift(day);
        println!("");
    }
}

fn gift(n: u8) {
    if n == 1 {
        println!("And a partridge in a pear tree");
    } else if n == 2 {
        println!("Two turtle doves,");
    } else if n == 3 {
        println!("Three french hens,");
    } else if n == 4 {
        println!("Four calling birds,");
    } else if n == 5 {
        println!("Five golden rings,");
    } else if n == 6 {
        println!("Six geese a-laying,");
    } else if n == 7 {
        println!("Seven swans a-swimming,");
    } else if n == 8 {
        println!("Eight maids a-milking,");
    } else if n == 9 {
        println!("Nine ladies dancing,")
    } else if n == 10 {
        println!("Ten lords a-leaping,")
    } else if n == 11 {
        println!("Eleven pipers piping,");
    } else if n == 12 {
        println!("Twelve drummers drumming,");
    }

    if n > 1 {
        gift(n-1);
    }
}