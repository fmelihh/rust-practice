fn main() {
    // variables();
    println!("Success");
}


fn variables() {
    //assingments
    let x: i32 = 5;
    assert_eq!(x, 5);

    // mutable keyword
    let mut y: i32 = 10;
    y += 10;
    assert_eq!(y, 20);

    // calling a function
    define_x();

    // shadowing
    let m: i32 = 10;
    {
        let m = 12;
        assert_eq!(m, 12);
    }
    assert_eq!(m, 10);

    // Destructuring
    let (d, f) = (1, 2);
    println!("{}", d);
    println!("{}", f);

    // singed integer = can represent both positive and negative integers
    // unsinged integer = always positive integers

    // default integers i32
    // default floats f64
}

// declaring a function
fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}
