fn main() {
    let x = 5;
    println!("X is {x}");
    // Can't do this 👇 
    // x = 4;
    let x = x*2;
    const SOME_CONSTANT:i32 = 5;
    {
        let x = x*2;
        println!("X is {x}");
    }

    println!("COnst is {SOME_CONSTANT}");

    println!("X is {x}");

    let mut y = 5;
    println!("Y is {y}");
    // Can do this bcoz, y is mutable
    y = 6;
    println!("Y is {y}");

    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Compund type ⚠️⚠️

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,_y,_z) = tup;

    println!("X is {x}");

    let x = {
        let y = 6;
        y*2
    };

    let x = "Hello World";

    println!("x is {x}");

    println!("Something {}",plus_one(x));

    // Control flow

    let x = if true {5} else {6}; 

}

fn plus_one(x:&str) -> &str{
    x
}
