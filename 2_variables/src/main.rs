fn main() {
    // a constant
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // a mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 5;
    let y = y + 1; // shadows the first y, but still uses the original value

    {
        let y = y * 2; // shadows the value, but only inside the {}
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is: {y}");

    // tuples - have a fixed size, and are allocated in the heap

    // a tuple is considered to be a single *compound* element!
    let tup = (500, 6.4, 1);

    // destructuring a tuple (similar to JS)
    let (x, y, z) = tup;
    println!("x = {x}, y = {y} and z = {z}");

    // accessing a value inside the tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}, {six_point_four}, {one}");

    // arrays - are stored in the stack and has a *fixed* size!
    let array = [1, 2, 3, 4, 5];
}
