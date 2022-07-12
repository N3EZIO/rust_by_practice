fn main() {
    println!("{:?}, world", define_x()); 
}

fn define_x() -> String {
    let x = String::from("hello");
    x
}