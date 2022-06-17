fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of x is: {}",x);
    println!("The value of y is: {}",y);
    println!("The value of z is: {}",z);
    
}
