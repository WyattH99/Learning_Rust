fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');
    
    let five = five();
    println!("five(): {five}");

    let x = plus_one(5);
    println!("plus_one: {x}");

    
    // Control Flow
    let number = 8;
    println!("number: {number}");
    if number < 5 {
        println!("number < 5: true");
    } else {
        println!("number < 5: false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is : {number}");
}

fn another_function(x: i32) {
    println!("Another function, x: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
