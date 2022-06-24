fn main() {
    // For loops
    // They are a little differente from other languages.println!
    
    for x in 0..10 { 
    // From 0 to 9, the right side is exclusive (..)
        println!("\nThe value of x is: {}", x);
    }

    //What if you wanted to write something which is more explicit?
    for y in 0..=9 {
    // The right side included now (..=)
        println!("\nThe value of y is: {}", y);
    }

    //With Iterators
    let z = [1, 2, 3];

    for val in z {
        println!("\nThe value is {}", val);
    }

}
