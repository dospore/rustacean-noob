use std::io;

fn main() {
    for _count in 1..6 {
        let mut temp = String::new();
        println!("Please enter a temperature in shitty American units");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid input temp set to 0"); 0.0 },
        };

        println!("Entered shitty American units: {} Fahrenheit", temp);

        let temp = (temp - 32.0) * 5.0/9.0;

        println!("Here is your temperature in regulare people units: {}\n", temp);
    }

    println!("ENOUGH TEMPERATURE CONVERSIONS!!!")
}
