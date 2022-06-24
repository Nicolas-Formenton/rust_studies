fn main() {
    //Loop
    //Multiply x till it is less than 5000
    //Execute the code WHILE the condition is true

    let mut x = 1;

    // How would be write this with the while construct?
    loop{
        x = x*2;
        
        if x > 5000{
            break;
        }

        println!("The value of x now is {}", x)
    }

    let mut y = 1;
    
    //4096
    while y < 5000 { // 8192 < 5000 - false
        y = y*2;
        println!("The value of y now is {}", y)
    }

    println!("Outside the loop");
}
