fn main() {
    //Tuples
    //Return a set of differente types of values
    let a = (5, "Hello", false);
    println!("The first two values are: {}; {}", a.0, a.1);

    // How do you acces multiple values at once?

    let b = (1, 2, 3);

    //Option 1
    //let val_one = b.0;
    //let val_two = b.1;
    //let val_three = b.2;

    //println!("The first three values are: {}; {}; {}. (Respectively)", val_one, val_two, val_three);

    //Option 2 (Also called destructuring)
    let (val_one, val_two, _) = b;
    println!("The first three values are: {}; {}; (Respectively)", val_one, val_two);

}
