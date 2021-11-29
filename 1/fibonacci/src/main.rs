use std::io;

fn fibonacci(num: u32) -> u32 {
    match num {
        1 | 2 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}

fn main() {
    loop {
        println!("Find n-th fibonacci number. (1,1,2,3,5,...)");

        println!("Please write down any natural number. (0 to quit)");

        let mut input_number = String::new();

        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to input");

        let input_number: u32 = match input_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input_number == 0 {
            break;
        }

        println!("n-th fibonacci number is: {}\n", fibonacci(input_number));
    }
}
