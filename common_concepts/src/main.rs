fn main() {
    let x = 5;

    {
        // The first x is shadowed by the second x.
        let x = 6;
        println!("x is {x}");
    }

    println!("x is {x}");
    // Prints
    // x is 6
    // x is 5

    let mut y = 5;
    y = 6;
    println!("y is {y}");

    // Can't change the type with mut - let can be 
    // can be used again in the same scope.

    let y = "hello";
    println!("y is {y}");



    // Constants are always immutable.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");


    // Scalar types:
    //   - Integers
    //      - Unsigned integers:
    //      - Unsigned 8-bit integer: u8 0..=255 (u8::MAX)
    //      - Unsigned 16-bit integer: u16 0..=65535 (u16::MAX)
    //      - Unsigned 32-bit integer: u32 0..=4294967295 (u32::MAX)
    //      - Unsigned 64-bit integer: u64 0..=18446744073709551615 (u64::MAX)
    //      - Unsigned 128-bit integer: u128 0..=340282366920938463463374607431768211455 (u128::MAX)
    //      - Unsigned size integer: usize 0..=usize::MAX (usize::MAX)
    //  - Signed integers:
    //       - Signed 8-bit integer: i8 -128..=127 (i8::MIN, i8::MAX)
    //       - Signed 16-bit integer: i16 -32768..=32767 (i16::MIN, i16::MAX)
    //       - Signed 32-bit integer: i32 -2147483648..=2147483647 (i32::MIN, i32::MAX)
    //       - Signed 64-bit integer: i64 -9223372036854775808..=9223372036854775807 (i64::MIN, i64::MAX)
    //       - Signed 128-bit integer: i128 -170141183460469231731687303715884105728..=170141183460469231731687303715884105727 (i128::MIN, i128::MAX)
    //       - Signed size integer: isize -isize::MIN..=isize::MAX (isize::MIN, isize::MAX)
    // - Floating-point numbers: f32, f64 (defaults to f64)
    // - Booleans: bool true, false (bool::TRUE, bool::FALSE)
    // - Characters: char 'a', 'b', 'c' (char::A, char::B, char::C)

    // Integer overflow
    // Integer overflow occurs when an integer operation results in a value that is too large or too small to be represented.
    // Rust will check for this at compile time, and will panic if it happens.
    let mut x: u8 = 255;
    // x = x + 1; - will panic
    
    // To wrap around the value, you can use the wrapping_add method.
    x = x.wrapping_add(1);
    println!("x is {x}"); // 0
    // or subtract
    x = x.wrapping_sub(1);
    println!("x is {x}"); // 255


    // Tuples
    // - Tuples are a collection of values of different types
    // - Tuples have a fixed size

    let tup: (i32, f64, char) = (500, 6.4, 'h');
    let (x, y, z) = tup;
    println!("x is {x}, y is {y}, z is {z}");

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x is {x}, y is {y}, z is {z}");

    // Arrays
    // Must be of the same type and fixed size.
    let arr: [i32; 3] = [1, 2, 3];
    let arr = [3; 5]; // [3, 3, 3, 3, 3]

    // Accessing elements of an array
    let first = arr[0];
    let second = arr[1];
    let third = arr[2];
    println!("first is {first}, second is {second}, third is {third}");

    // it will panic if you try to access an element out of bounds.
    // let fourth = arr[3]; // will panic

    another_function();
    // doesn't matter it was defined before or after the main function.

    print_labeled_measurement(5, 'h');

    // Statements and Expressions
    // Statements are instructions that perform some action and do not return a value.
    let y = 6;
    // Expressions are combinations of values and operators that produce a value.
    let z = y + 1; // y + 1 is an expression that produces a value.

    // You can't do this
    // let x = (let y = 6); - because let y = 6 is a statement, not an expression. it doesn't return a value.

    // Expressions do not include trailing semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will not return a value.
    let y = {
        let x = 3;
        x + 1
    };
    // Not no ; on the last line, because it would be a statement, not an expression.
    println!("y is {y}");


    // Functions with return values

    let x = five();
    println!("x is {x}");

    let x = plus_one(x);
    println!("x is {x}");

    println!("{}", plus_one(five()));

    /*
    Multipleline comments are like this.
    x is 5
    x is 6
    x is 6
    */

    // IF EXPRESSIONS - no parentheses around the condition!
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {    
        println!("condition was false");
    }
    // must be a bool too (true or false)
    // can do if number {} like in JS - no truthy/falsy values like in JS

    // else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let statements can be used to assign a value to a variable based on a condition.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {number}");

    // let statements can be used to assign a value to a variable based on a condition.
    let number = if condition { 5 } else { 6 };
    // the number var here is typed to the last expression in the if block.

    // let number = if condition { 5 } else { "six" }; - will panic, because the else block must return the same type as the if block.

    // LOOP EXPRESSIONS
    // loop expressions are a way to repeat a block of code until a condition is met.
    // loop expressions are expressions, they return a value.
    // loop expressions are used to repeat a block of code until a condition is met.
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("count is {count}");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // this is the value that will be returned from the loop.
        }
    };

    // You can also return from inside a loop.
    // While break only exits the current loop
    // return always exits the current function.

    // loop labels

    // begin with a single quote and a colon, like this: 'label_name:
    let mut count = 0;
    'counting_up: loop {
        println!("counting up: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 10 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    
    println!("End count = {count}");
    
    println!("result is {result}");

    // while loops

    let mut number = 3;
    while number != 0 { // again no parentheses around the condition!
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loops

    let a = [10, 20, 30, 40, 50];
    for element in a { // again no parentheses around the condition!
        println!("the value is: {element}");
    }

    // range loop

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
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
// Again no ; on the last line, because it would be a statement, not an expression.
// No need for `return` keyword, because the last expression is the return value.