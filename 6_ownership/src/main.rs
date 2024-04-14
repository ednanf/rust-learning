fn main() {
    /*
    The String type

    - Manages data allocated on the heap
    - Stores an amount of text that is unknown at compile time
    - This kind of string can be mutated

    */

    // Scoping
    {
        let s = String::from("hello"); // s is valid from this point forward
                                       // still valid here
    }
    // scope is over and s is no longer valid -> the memory is freed

    // Moving a value (similar to a shallow copy)
    let s1 = String::from("Hello");
    let s2 = s1; // the value *moved from s1 and s1 no longer is valid!

    // Cloning a value (similar to a deep copy) - more expensive
    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // Stack-only data: copy
    // If we are using only integers, we can make a shallow copy instead of a move
    // because an integer's size is always know
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
