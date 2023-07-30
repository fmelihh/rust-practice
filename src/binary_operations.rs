use std::mem::size_of_val;

pub fn binary() {
    // boolean expressions
    assert!(true && false == false);
    assert!(true || false == true);
    assert_eq!(!true, false);

    // measure variable size in memory
    let c1: char = 'A'; // 4 bytes
    println!("{}", size_of_val(&c1));
    assert_eq!(size_of_val(&c1), 4);

    // if statement
    let t: bool = true;
    if t {
        println!("inside the if");
    }

    //unit type
    let _v: () = ();
    // if some function doesnt return anything compiler will be return a unit type ()

    assert_eq!(size_of_val(&_v), 0);
    // unit types have af zero memory size.
}