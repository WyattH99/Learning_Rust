fn main() {
    println!("Hello, world!");

    /* String Literals and Scopes */
    {
        let s = "hello";
        println!("{s}");
    }
    // s is now out of scope
    //println!("{s}"); // This throws an error "not found in this scope


    /* String Type can be made from a String Literal */
    // String Types have ptr, len, and capacity variables
    // Dynamic Memory and is stored on Heap
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str appends a literal to a str
    println!("{s}");


    // C++ pattern, Resource Acquisition Is Initialization (RAII)
    // When a variable goes out of Scope Rust Automatically calls the "drop" function
    // and cleans up the heap memory
    let s1 = String::from("hello");
    //let s2 = s1; // s1 moves ownership to s2, NOT making a copy
    //println!("{s1}, world!"); // ERROR: norrow of moved value: 's1'
    let s2 = s1.clone(); // Makes a "deep" copy
    println!("s1 = {s1}, s2 = {s2}");


    /* Ownership and Functions */
    let s = String::from("hello");
    takes_ownership(s);
    // Can't use s anymore as it was dropped after the function call

    let x = 5;
    makes_copy(x);
    println!("makes_copy: {x}"); // This is still valid as integers are on the stack
    // and drop is not called on them


    /* Ownership Returns and Scopes */
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    //println!("s2: {s2}"); s2 was dropped on the last line
    println!("s3: {s3}");


    /* Tuple lets multiple ownership returns */
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // Tuple returned
    println!("The length of '{s2}' is {len}.");

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
// some_string goes out of scope and drop is called

fn makes_copy(some_integer: i32) { 
    println!("{some_integer}");
}
// some_integer goes out of scope but i32 is not on heap so drop is not called

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // End of Function lines w/o a ";" are returned at the end of scope
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // tuple returns ownership of s and length
}






