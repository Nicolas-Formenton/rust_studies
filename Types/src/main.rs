//Don't need to know how it works for now!
trait TypeDebug{
    fn print_type(&self);
}
impl<T> TypeDebug for T {
    fn print_type(&self) {
        println!("The type is '{}'", std::any::type_name::<T>());
    }
}

fn main() {
    let x: i32 = 5; //Not necessarily needs to put the i32. //Signed 32bit integer.
    println!("\nThe value of the variable is {}", x);
    x.print_type(); //Type Inference, only works due to the code above.
    //x = "hello world!"; //Static types
    let y = 3.14;
    y.print_type();

    let z = 'c';
    
    z.print_type();

    let h = true; //boolean type (True or False)
    h.print_type();

    let a = false;  //Just change values here to change the answer below.
    let b = true;

    if a { // if a is true
        println!("a is true");
    }
    if b {
        println!("b is true\n");
    }
}
