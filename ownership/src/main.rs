fn main() {
    /*
        ----- Ownership  rules in rust (Important to know) ------
        1. Each value in Rust has a variable that's called it's owner
        2. There can only be one owner at a time
        3. when the owner goes out of scope, the value will be dropped
    */

    /*
        ----- The Rules of References  ------
        1. At any given time, you can have either one mutable reference
            or any number of immutable reference
        2. References must always be valid
    */

    {// s is not valid at this point

        let s:&str = "hello"; // s is valid at this point
        // do stuffs with s
        println!("{}", s);
    }// scope ends, and s is dropped(invalid again)

    // trying out rust move
    {
        let x:i32 = 5;
        let _y:i32 = x; // copy, rust will copy, as it is primitive and it's quite on the stack;

        let s:String = String::from("hello");
        /*
            s                       heap
            ptr | ->                index | value
            len | 4                   0   |  h
            capacity | 5              1   |  e
                                      2   |  l
                                      3   |  l
                                      4   |  0
        */

        let s1:String = s; // move had occured here
        // rust won't make a shallow copy, or clone it
        // instead it will transfer ownership

        // to clone
        let _s2:String = s1.clone(); // creates a clone of previous variable value
        // and still keep that variable's ownership intact

    }
    
    // Using references
    {
        let s:String = String::from("hello world"); // this will move ownership to function
    takes_ownership(s);

    let s:&str = "56string";// this will pass a copy to funtion
    takes_copy(s);

    let s:String = give_ownership(); // gains ownership of the value returned
    println!("{}",s);
    let s3:String = take_and_give_back_ownership(s);
    println!("Gave, and received: {}",s3);
    let lent:usize = calculate_length(&s3);
    println!("Length of `{}` is length {}", s3, lent);

    // let new_lent:usize = calculate_length(&s3);
    // println!("New length of `{}` is length {}", s3, lent);

    let mut s4:String = String::from("Hello");
    change(&mut s4);

    // You can't borrow mutable references more than once in rust
    }


    /* 
        Slices

        slices makes us reference a contiguos sequence of elements
        within a collection, instead of referencing the entire collection
    */
    {
        let s:String = String::from("Hello world");

        // let hello:&str = &s[0..5];
        // let world:&str = &s[6..11];
        // let hello:&str = &s[..5]; // span from start to an index
        // let world:&str = &s[6..]; // span from an index to the end;
        // let all:&str = &s[..]; // span the entire sequence;

        let word:&str = first_word(&s);
        println!("{}", word);

        //s.clear() // clear string

        let a = [1,2,3,4,6];
        let slice = &a[0..4];
        println!("{}", slice[0]);
    }
    
}


// Ownership with functions
fn takes_copy(some_string:&str){
    println!("{}",some_string);
}

fn takes_ownership(some_string:String){
    println!("{}",some_string);
}

fn give_ownership() -> String{
    // creates a string, and transfers the ownership
    // to the variable(scope) where the function was called
    let new_string:String = String::from("New String");

    new_string
}

fn take_and_give_back_ownership(a_string:String) -> String{

    a_string
}



// Using References
fn calculate_length(s:&String)-> usize{
    // Passing references to functions is called borrowing
    // by default are immutable
    let length:usize = s.len(); // len() returns the length of a string

    length
}

// fn add_and_calculate_length(s:&String)-> usize{
//     // Passing references to functions is called borrowing
//     // by default are immutable
//     // s.push_str("world");
//     let length:usize = s.len(); // len() returns the length of a string

//     length
// }

fn change(s:&mut String){
    s.push_str(", world");
}


fn first_word(s:&String) -> &str{
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}