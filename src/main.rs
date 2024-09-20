use std::io;

fn main() {
    let mut conversion_type = String::new();
    let mut input_temp = String::new();

    println!("Please enter what you would like to convert to.
        Fahrenheit (F) or Celsius (C):");

    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Failed to read line");

    let conversion_type: char = match conversion_type.trim().parse() {
        Ok(char) => char,
        Err(_) => return,
    };

    println!("Please enter the temperature you would like to convert");

    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    let input_temp: f64 = match input_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if conversion_type == 'C' {
        fahrenheit_to_celcius(input_temp);
    } else if conversion_type == 'F' {
        celcius_to_fahrenheit(input_temp);
    } else {
        println!("Please enter 'F' or 'C'")
    }
    
}

fn fahrenheit_to_celcius(input_temp: f64) {
    let temp_c: f64 = (5.0/9.0) * (input_temp - 32.0);
    println!("Fahrenheit:{input_temp}° --> Celcius:{temp_c}°");
}

fn celcius_to_fahrenheit(input_temp: f64) {
    let temp_f: f64 = (9.0/5.0) * (input_temp) + 32.0;
    println!("Celcius:{input_temp}° --> Fahrenheit:{temp_f}°");
}

