// Hour 3 Summary Practice Code
fn main() {
    // Variable Binding (Immutable)
    println!("Variable binfing immutable");
    let (x, y) = (100, 200);
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!(" ");

    // Variable Binding (Mutable)
    println!("Variable binfing mutable");
    let mut a = 100;
    let mut b = 200;
    a = a + 300;
    b = b + 400;
    println!("Finally a is {}", a);
    println!("Finally b is {}", b);
    println!(" ");

    // String Assignment
    println!("String assignment");
    let x = "hello".to_string();
    let y = String::from("hello");
    let z: &str = "hello";
    print!("{} {} {}\n", x, y, z);
    println!(" ");

    // Arithmetic Operators
    println!("Arithmetic Operators");
    println!("10 + 2 = {}", 10 + 2);
    println!("10 - 2 = {}", 10 - 2);
    println!("10 * 2 = {}", 10 * 2);
    println!("10 / 2 = {}", 10 / 2);
    println!("10 % 2 = {}", 10 % 2);
    println!(" ");

    // Logical Operators
    println!("Logical Operators");
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    println!(" ");

    // Comparison Operators
    println!("Comparison Operators");
    let x: i32 = 100;
    let y: i32 = 200;
    println!("x is greater than y : {}", x > y);
    println!("x is less than y : {}", x < y);
    println!("x is unequal to y : {}", x != y);
    println!("x is greater/equal to y : {}", x >= y);
    println!("x is less/equal to y : {}", x <= y);
    println!("x is completely equal to y : {}", x == y);
    println!(" ");

    // Array - Method 1
    println!("Array method 1");
    let mut a: [i32; 4] = [8; 4];
    a[1] = 10;
    a[2] = 20;
    println!("{} {} {} {}", a[0], a[1], a[2], a[3]);
    println!(" ");

    // Array - Method 2
    println!("Array method 2");
    let b: [f32; 4] = [0.1, 0.2, 0.3, 0.4];
    println!("{} {} {} {}", b[0], b[1], b[2], b[3]);
    println!(" ");

    // Slice
    println!("Slice");
    let a = [0, 10, 20, 30, 40, 50, 60];
    let slice = &a[2..5];
    println!("{}", slice[0]);
    println!("{}", slice[1]);
    println!("{}", slice[2]);
}
