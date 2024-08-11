use std::io;


fn main() {
    // Take user input
    // Convert
    // Output
    // Infinte Loop


    println!("Give me Fahrenheit, I give you Celsius!");

    loop {
        let mut fahrenheit = String::new();

        println!("Enter a number: ");

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: i32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let celsius = f_to_c(fahrenheit);

        println!("{fahrenheit}°f = {celsius}°c");
    }

}

fn f_to_c(f: i32) -> i32 {
    (f-32)*5/9
}
