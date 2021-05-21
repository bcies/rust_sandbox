use std::io;

fn main() {
    println!("Welcome to the Fahrenheit => Celsius converter.\nWhat temperature would you like to convert?");

    loop {
        let mut temperature = String::new();
        
        io::stdin().read_line(&mut temperature).expect("Failed to read line.");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };


        let celsius = (temperature - 32.0) * (5.0 / 9.0);

        println!("{} Fahrenheit is {} Celsius.", temperature, celsius);

        break;
        
    }

}
