//1
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 


//2
// Make it work
fn main() {
    let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}

//3
// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
} 

//4
// Make it work
fn main() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}


//5
// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3); // _ applies for shadowing too
    assert_eq!((), implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


//6
// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); //size of empty tuple is 0

    println!("Success!");
} 

