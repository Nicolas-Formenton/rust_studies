fn main() {
    //Arrays
    // A set of the same type of values.

    let a = [1, 2, 3, 4];
    println!("The first value is: {}", a[0]);


    let b: [i32; 10] = [0; 10];
    println!("The array is: {:?}", b);
    // {:?} Faz uma linha de sequência; 
    // {:#?} Faz uma coluna com a sequência.

    
    let mut c = [0];
    println!("The value is: {}", c[0]);
    
    c[0] = 1;
    println!("The value is: {}", c[0]);
}
