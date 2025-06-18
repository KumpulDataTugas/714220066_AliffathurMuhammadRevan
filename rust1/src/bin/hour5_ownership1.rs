fn main() {
    let x = String::from("try");
    let _y = x;
    println!("{}", x); // This line causes an error due to move
}