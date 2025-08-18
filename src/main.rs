use std::io;
fn main() {
    println!("Welcome to temperature converter! ");
    loop {
        println!("Choose an option from below:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");
        println!("Enter an option: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input");

        let _option = match input1.trim() {
            "1" => {
                println!("Enter Fahrenheit to convert to Celsius");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read input");
                let _x = 5 / 9;
                let input2: f64 = match input2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input is not a number!");
                        continue;
                    },
                };
                let res = (input2 - 32.0) * 5.0 / 9.0;
                println!("{input2} Fahrenheit in Celsius is: {res:.2}°");
                println!("Press ENTER to continue");
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "2" => {
                println!("Enter Celsius to convert to Fahrenheit");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read input");

                let input2: f64 = match input2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input is not a number!");
                        continue;
                    },
                };
                let res = (input2 * 1.8) + 32.0;
                println!("{input2} Celsius in Fahrenheit is: {res:.2}°");
                println!("Press ENTER to continue...");
                io::stdin().read_line(&mut String::new()).unwrap();
            }
            "3" => {
                println!("Bye!");
                break;
            },
            _ => {
                println!("Invalid option!");
                continue;
            },
        };
    }
}
