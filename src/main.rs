// Single-line comment

/* 
Multi-line
comment
*/ 

// Compile on command line `cargo run`

// Global Variables

/*
Constants are global variables that cannot be changed
They are always declared outside of the main function
Must declare type of variable
Declare a constant: const NAME:type = value;
*/

const GLOBAL_VARIABLE:i32 = 10;

/*
Data Types - https://doc.rust-lang.org/book/ch03-02-data-types.html
Important to specify data types for readability and performance optimization

Integer 

8bit - 128bit
Use unsigned when number will never be negative
Use f for floating-point numbers
i8 | u16 | f32
isize/usize used for indexing collections
Defaults to i32 or f64
let x: i64 = 1234567890;

Boolean

let t: bool = true;
*/

// Enum declaration (addressed below)

/*
enum Direction {
    Up,
    Down,
    Left,
    Right
}
*/

// fn main is part of every Rust project
fn main() {

 // Print to console when program is compiled
    println!("Hello, World!");

 /* 
Print a variable within a string with brackets {} in place of the variable name
After the string is closed place a comma, followed by the name/s of the variable/s used
let one = 1; let two = 2; let three = 3;
 println!("Counting down: {} {} {}..., three, two, one ");
*/

    println!("The global variable is {}", GLOBAL_VARIABLE);

// Declare a variable: let name = value;

    let x = 45;

 // let mut x = 45;

    println!("x = {}", x);

 // x = 60;

 // println!("Now x = {}", x);

/* 
All Rust variables are non-mutable 
Uncomment lines 74 & 76
`cargo run` throws error
Uncomment line 70
Comment out line 68
`cargo run` prints both values of x
*/

// If Statements

    let n = 35;

    if n == 20 {
        println!("{} is equal to the magic number", n);
    } else if n < 30 {
        println!("{} is less than the magic number", n)
    } else {
        println!("{} is greater than the magic number", n);
    }     

 // Loop

    let mut n = 0;

    loop {
        n += 1;
        if n > 2 && n < 99 {
            // Skip iteration
            continue;
        }
        if n > 100 {
            // Stop iteration
            break;
        }
        println!("Loop: {}", n);
    }

 // While Loop

    let mut w = 1; 

    while w <= 50 {
            if w % 17 == 0 {
            println!("{} is divisible by 17!", w);
        }
        w += 1;
    }

    // For Loop

    for i in 1..4 {
        println!("Count:{}", i);
    }

// Declare a vector and loop through strings

    let animals  = vec!["Frog","Dog","Cat"];

// Must include .iter() method to iterate through vector and .enumerate() method to print index
    for (index, a) in animals.iter().enumerate() {
            println!("{}. {}", index, a);
        }
    
 // Accessing Enums (defined outside of main function, line 42)

/* 
Uncomment lines 42-48 & 150-159
`run cargo` throws 3 default warnings for dead code (unused directions)
let name:enum_Name = enum_name::enum_value
*/

    // let player_direction:Direction = Direction::Up;

 /*
    match player_direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
*/

 // Tuples/Nested Tuples

    let tuple = (20, 30, (40, 50), 60);

// Zero-indexed, prints "30" then "50"

    println!("Tuple 1 ={}", tuple.1);
    println!("Tuple 2.1 = {}", (tuple.2).1);

 // Define a tuple equal to tuple above and print switched values

    let (a, b, c, d) = tuple;

//   prints 20, 30, 40 and 50, 60 
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {} and {}", c.0, c.1);
    println!("d is {}", d);

 // Code Block Scope

    let z = 10;

    {
        let z_scoped = 11;
        println!("z = {}, z_scoped = {}", z, z_scoped);
    }

    // println!("{} {}", z, z_scoped)  

 // Uncomment Line 189, throws error because z_scoped is only valid inside code block

 // Shadowing

    let mut s = 10;
    println!("s = {} before code block ", s);

    {
        // Reassigns value of s to 15 that is valid outside of code block
        s = 15;

        // Comment out Line 199, uncomment Line 202, now  s = 15 only inside of code block
        // let s = 15;
        println!("s = {} inside code block", s);
    }

    println!("s = {} after code block", s);

// References - refer to same data with multiple variables

    let mut f = 10;

    /* 
    Ampersand indicates reference to variable that follows
    Reference below is immutable unless otherwise specified
    */
    
    // let f_ref = &f;

    //

    //

    {
        let f_ref = &mut f;
        *f_ref += 1;
    }

    // Updated values for f and f_ref printed to console
    println!("f is {}", f);
    // println!("f_ref is {}", f_ref);
    
    /* 
    Uncomment line 205, throws error
    Comment lines 190 and 210 (+4 after formating)
    Uncomment lines 193 and 212 (+4)
    Run again, throws new error "mutable borrow occurs"
    Cannot be mutable and immutable simultaneously
    Uncomment line 195 and format code block
    Cut and paste lines 193 and 212 (+4) into code block
    Run again, throws new error "f_ref not found in this scope"
    Comment out line 203, notice f prints 11 even though f_ref is not in scope
    */

    // f_ref += 1;


    
    // Function tests (Defined below fn main)
    print_numbers(3);

    // How do I println! input (35), "num" throws error
    if is_even(35) {
        println!("The number is even!");
    } else {
        println!("The number is odd!");
    }

// End fn main    
}

// Functions

fn print_numbers(num: u32) {
    for n in 1..num + 1 {
        println!("Function number: {}", n)
    }
}

fn is_even (num: u32) -> bool {
    return num % 2 == 0;
}