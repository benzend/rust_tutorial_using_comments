use std::io;

fn main() {
    loop {
        // * `mut` allows mutation
        // * String::new(); sets you up with an empty *string*
        // * in which, since Strings are mutable unlike literals, 
        // * they get assigned to the heap
        let mut range_start = String::new();
        let mut range_end = String::new();
    
        // * Note: the arguments used here are *literals*, these 
        // * are stored on the stack since they are immutable
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

        // * Rust doesn't handle ranges that go from larger to smaller e.g. 5..1
        // * the only way to handle this is by converting it to 1..5 and calling .rev()
        if range_start > range_end {
            for n in (range_end..range_start + 1).rev() {
                println!("{}", n);
            };
        } else {
            for n in range_start..range_end + 1 {
                println!("{}", n);
            };
        };
    }

}

fn good_bye() {
    println!("Goodbye");

    // * Exit the application
    std::process::exit(0x0100);
}
