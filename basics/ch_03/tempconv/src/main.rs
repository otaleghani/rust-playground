use std::io;

fn main() {
    println!("Convert °C to °F");

    loop {
        println!("\nInsert °C");
        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        if celsius.eq("quit\n") {
            break;
        }

        let celsius: f64 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("ERR: {err}");
                0.0
            }
        };

        let fahrenheit = celsius * 1.8 + 32.0;
        println!("Converted value: {celsius}");
        println!("Converted value: {fahrenheit}\n");
    }
}
