fn main() {
    // returning values from loops - add the value after "break"
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returning the value after "break"
        }
    };

    println!("The result is {result}");

    // loop labels - used to identify loops in loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!(
        "End count = {count}
    "
    );

    // conditional loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping through a collection with "for"
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // runing a code a limited amount of times with a for loop and a range
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!");
}
