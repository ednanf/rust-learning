fn main() {
    let number = 3;

    // if expressions, type must match
    if number < 5 {
        println!("condition was true, the number is: {number}");
    } else if number > 5 {
        println!("condition was false, the number is: {number}");
    } else {
        return;
    }

    // if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is: {number}")
}
