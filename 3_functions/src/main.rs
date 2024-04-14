fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    // a statement does not return a value, and it ends with a semicolon
    let statement = 5;
    println!("{statement}");

    // expressions evaluate to a value. The content between {} get bound to the variable expr
    let expr = {
        let x = 3;

        // expressions don't end with a semicolon
        x + 1
    };
    println!("{expr}");

    let fn_with_return_value = five();
    println!("The value is: {fn_with_return_value}");
}

// we must declare the type of each parameter
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// functions with return values - the type must be declared after an arrow!
fn five() -> i32 {
    // no semicolon because it's a value we will return
    5
}
