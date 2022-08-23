fn main() {
    // Variables are immutable by default in rust
    let  mut x: i32 = 5;

    println!("The value of x is: {}",x);
    
    x = 16;

    println!("The value of x is: {}",x);

    // Const are values in the program that will never change
    // const cannot be mutated
    // const must be given a type
    // const cannot be assigned to return values
    const SUBSCRIBE_COUNT: u32 = 100_000; // it's common practise to make constants uppercase

    println!("Subscriptions: {}",SUBSCRIBE_COUNT);


    // Shadowing is creating a new variable using an existing name
    // Shdowing gives two advantages
    // 1 - it preserves mutability
    // 2 - it allows type changing


    // Data types
    // There are two data types, Scalar data types and compound data types
    /* 
        Scalar datatypes: represent a single value
            Rust has four main datatypes
            - integers: 
                are numbers without a fraction
                they are:
                    length      | signed(positive or negative) | unsigned(positive only)
                    8-bits      |            i8                |         u8
                    16-bits     |            i16               |        u16
                    32-bits     |            i32               |        u32
                    64-bits     |            i64               |        u64
                    128-bits    |           i128               |        u128
                    arch        |           isize              |        usize
                
                Rust defaults to i32
                
            - floating point numbers
                are numbers with decimal points

                Rust defaults to f64
            - booleans
                represents the value true or false

            - characters
                represents a unicode character in a single quote

        Compound datatypes: represent a group of values
    */   
/* 

    // Integers
    let a:i32 = 98_222; // Decimal(base 10)
    let b:i32 = 0xff; // Hex(base 16)
    let c:i32 = 0o77; // Octal(base 8)
    let d:i32 = 0b1111_000; // Binary(base 2)
    let e:u8 = b'A'; // Byte(u8 only)
    /* 
        Integer overflow means, if a number is greater than the maximum
        value a type can hold, in debug, rust will panic, in build
        rust will wrap around to the minimum values

        let f:i8 = 255;

        if f changes to 256, it overflows and becomes 0(in build, otherwise panic in debug)
    */


    // Floating point numbers
    let f:f64 = 2.0;
    let g:f32 = 3.0;

    // addition
    let sum:i32 = 5+10;
    // subtraction
    let difference:f64 = 95.5 - 4.3;
    // Multiplication
    let product: i32 = 4 *30;
    // division
    let quotient: f64 = 56.7/32.2;
    // remainder
    let remainder: i32 = 43 % 5;


    // Booleans
    let t: bool = true;
    let v: bool = false;

    // Character
    let cr:char = 'z';
    let z:char = 'Z';
    let heart_eye_cat:char = '😻';
 */

    // Compound Types
    /* 
        Types that represents a group of values

        Tuple types: 
            a fixed sized array of related data that could be of any type.
            we can get values from a tuple in two ways,
            - destructuring
            - dot notation

            tuples and arrays both start with index 0

        Array types:
            they are like tuple, except that, values must
            be of same type, and is enclosed in a square bracket.

            we get values from an array by index, e.g arr[index],
    */
/*     let tup:(&str, i32) = ("Let's get rusty!", 100_000);
    // getting values from tuple
    let (channel,sub_count) = tup;

    let sub_count_2 = tup.1;

    // Array
    let error_codes = [200,404,400];

    let not_found:i32 = error_codes[1]; // access a value by index in array;
    // let x = error_codes[3]; // memory unsafe, rust will panic!;
    let byte = [0;8]; // creates an array of length 8, with 0s as values like [0,0,0,0,0,0,0,0]

 */

    let sum = my_func(5,6);
    println!("Sum is: {}",sum);


    
    /* 
        Control Flow

        if statements (if, else if, else)
            the condition(in if and else if) must be boolean
        
        Loops
            the basic type is the one we make with the loop keyword
            
            loop{ // can be assigned to a variable, if it will return a value

                // runs statements in it scope forever
                // use break statement to break the loop
            }
        
            While (condition){
                // executes block while condition is true
            }

            for iteration {
                // executes block through iteration
            }
    */
    // Control Flow
    let number:i32 = 5;
    if number < 10{
        println!("First condition was true");
    }
    else if number < 22{
        println!("Second condition was true");
    }
    else{
        println!("Condition was false")
    }

    // tenary control flow
    let condition:bool = true;
    let val:i32 = if condition {5} else { 6};
    println!("Val: {}",val);


    // Loops
    let mut counter:i32 = 0;
    let result:i32 = loop{ // assig a loop to a variable to get it returned value
        println!("Again!"); // use CTRL+C to break
        // break; // stop the loop
        counter += 1;

        if counter == 10{
            break counter; // breaks the loop and return counter;
        }
    };

    println!("LIFTOFF!!! {}", result);

    println!("Run while loop on {}", counter);

    while counter > 0{
        println!("{}!",counter);

        counter -= 1;
    }

    println!("Done!");

    let iter = [1,2,3,4,5,7];

    for element in iter.iter(){
        println!("value is: {}",element);
    }

    // over a range of numbers
    for num in 1..10{ //last number exclusive
        println!("num is: {}",num);
    }
    

    
}


/* 
    Functions are defined using the fn keyword.
        we can receive parameters in functions by specifying the name and type in the parenthesis
        we can return a value from a function in two ways
        - using the return statement
            function must specify a return type,

    inside of a function, the last statement(without semi-colon) is implicitly returned

    In rust we can think of a piece of code as a statement, or as an expression
    statements performs some actions, but doesn't return a value
    expressions performs some actions, and returns a value
*/

fn my_func(x:i32, y:i32) -> i32 {
    println!("Another Function with parameters: x{}, y{}", x,y); // statement, returns no value

    let sum:i32 = x + y; // expression, returns a value

    // returning by return statement
    // return sum;
    sum // this is same as return sum;, no semi-colon
}


