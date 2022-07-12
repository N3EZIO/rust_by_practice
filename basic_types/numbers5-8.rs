//5
// Fix errors and panics to make it work
fn main() {
    let v1 = 247_u8 + 8_u8;
    let v2 = i8::checked_add(i8::MAX-8, 8).unwrap();
    println!("{},{}",v1,v2);
 }

 
 //6
// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}


//7
// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}


//8
fn main() {
    assert!(0.1_f32+0.2_f32 == 0.3_f32);

    println!("Success!");
}

//9
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 97..=122 {
        println!("{}",c);
    }
}

//10
// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}


//11
// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1);
    assert!(1i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 == 3.0_f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

