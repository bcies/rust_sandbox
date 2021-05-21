use std::io;

fn main() {
    println!("Which fibonacci number would you like?");

    let number: u32;
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line.");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        break;
    }
    match number {
        0 => {
            println!("Fibonacci number 0 is 0!");
        },
        1 => {
            println!("Fibonacci number 1 is 1!");
        },
        _ => {
            let mut fibonacci: i128 = 0;
            let mut past_fib: [i128; 2] = [0, 1];
            let mut index = 2;
        
            while index <= number {
                fibonacci = past_fib[0] + past_fib[1];

                past_fib[0] = past_fib[1];

                past_fib[1] = fibonacci;
        
                index += 1;
            }
        
            println!("Fibnonacci number {} is {}!", number, fibonacci);
        }
    }
}
