use std::io;

fn main() {
    loop {
        // * `mut` allows mutation
        let mut range_start = String::new();
        let mut range_end = String::new();
    
        println!("Please select a range of numbers.");
        println!("From integer: ");
    
        // * Takes input from the console
        io::stdin()
            .read_line(&mut range_start)
            .expect("Need a string");

        // * If user inputs "exit" leave program
        if range_start.trim() == "exit" {
            good_bye();
        };
    
        println!("To integer: ");
    
        // * Takes input from the console
        io::stdin()
            .read_line(&mut range_end)
            .expect("Need a string");

        // * If user inputs "exit" leave program
        if range_end.trim() == "exit" {
            good_bye();
        };
    
        // * cleanup strings (trim) and convert to integer (parse)
        let range_start: i16 = range_start.trim().parse().expect("To be an integer");
        let range_end: i16 = range_end.trim().parse().expect("To be an integer");

        println!("You selected range: {} - {}", range_start, range_end);
    
        println!("Now processing...");

        let is_negative: bool = if range_start < 0 { true } else { false };

        // * Make negative ranges into positive 
        let range_start: i16 = if range_start < 0 { range_start * -1 } else { range_start };
        let range_end: i16 = if range_end < 0 { range_end * -1 } else { range_end };

        // * Rust doesn't handle ranges that go from smaller to larger e.g. 5..1
        // * the only way to handle this is by converting it to 1..5 and running .rev() on it
        if range_start > range_end {
            for n in (range_end..range_start + 1).rev() {
                println!("{}{}", if is_negative { '-' } else { ' ' }, n);
            };
        } else {
            for n in range_start..range_end + 1 {
                println!("{}{}", if is_negative { '-' } else { ' ' }, n);
            };
        };
    }

}

fn good_bye() {
    println!("Goodbye");
    std::process::exit(0x0100);
}
