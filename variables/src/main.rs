fn main() {
    // Constants
    //let mut x = 5;
    //println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");


    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
   

    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");

    let float64 = 2.0; // f64 by default
    let float32: f32 = 3.0; // f32
    
    let t = true; // booleans
    let f: bool = false;

    let c = 'z'; // char variables have single quotes ' '
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tup cannot change length
    let (x, y, z) = tup;
    println!("The value of y deconstructed from tup is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Tuple access using a period: {five_hundred}, {six_point_four}, {one}");

    let a = [1, 2, 3, 4, 5]; // array on the stack, not the heap!
                             // arrays cannot change length
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // = [3, 3, 3, 3, 3]
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

}
