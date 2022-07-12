//1
// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;
    
    let z = 10; // Type of z i32

    println!("Success!");
}


//2
//  Fill the blank
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

//3
// Modify `assert_eq!` to make it work
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


//4
// Fill the blanks to make it work
fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}


