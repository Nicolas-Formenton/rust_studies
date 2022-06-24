fn main() {
    // Match statement
    // Mix of switch and if

    let x = 1;

    match x {
        1 => println!("Value of x is 1"),
        2 => println!("Value of x is 2"),
        _ => println!("Value of x is invalid"),
    }

    //Exhaustive matches!

    let a = false;
    let b = true;

    //Possible values:
    // true, true
    // false, false
    // false, true
    // true, false

    // This does NOT cover al possible cases!
//    if a && b {
//        println!("A and B are true");
//    } else if !a && !b {
//        println!("A and B are false");
//    } else if a && !b {
//        println!("A is true and B is false");
//    } else if !a && b {
//        println!("A is false and B is true");
//    }

    match (a, b) {
        (true, true) => println!("A and B are true"),
        (false, false) => println!("A and B are false"),
        (true, false) => println!("A is true and B is false"),
        //(false, true) => println!("A is false and B is true"),
        _ => println!("Invalid Combination"),
    }
}
