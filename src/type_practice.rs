// import statement
use std::ops::{Range, RangeInclusive};


pub fn types() {
    // type convertions
    // you can use as keyword or you can use underscre_type syntax"
    let v: i16 = 3_i8 as i16;

    // If we dont explicitly assing a type to variable, then the compiler will infer one for us.
    // default integer type is i32

    // max value of data type
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    let v2 :i16 = i16::checked_add(251, 8).unwrap();
    println!("{}", v2);

    let c = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert_eq!(c, 1597);

    // f64 default float type

    // for loop
    for c in 'a'..='z' {
        println!("{}", c);
    }

    // for loop with char to ascii type converting
    for c in 'a'..='z' {
        println!("{}", c as u8);
    }

    assert_eq!((1..5), Range{start:1, end:5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

}