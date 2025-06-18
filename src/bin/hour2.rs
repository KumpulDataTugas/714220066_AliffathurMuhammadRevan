const MAX_POINTS: i32 = 100;

fn main() {
    let int_var: i32 = 100;
    let float_var: f32 = 99.99;
    let boolean_var: bool = true;
    let char_var: char = 'A';
    let string_var: String = "Rust is awesome!".to_string();
    let array_var32: [i32; 3] = [1, 2, 3];
    let array_var64: [i64; 64] = [1; 64];
    let slice_var = &array_var32[0..2];
    let tuple_var = (42, "tuple", 3.14);

    println!("Integer: {}", int_var);
    println!("Float: {}", float_var);
    println!("Boolean: {}", boolean_var);
    println!("Char: {}", char_var);
    println!("String: {}", string_var);
    println!("Array 32 Bit: {:?}", array_var32);
    println!("Array 64 Bit: {:?}", array_var64);
    println!("Slice: {:?}", slice_var);
    println!("Tuple: {:?}", tuple_var);
    println!("MAX_POINTS constant: {}", MAX_POINTS);

    let converted: i32 = float_var as i32;
    println!("Converted float to int: {}", converted);

    print_sum(10, 20);
    let result = add_200(100);
    println!("Result of add_200: {}", result);

    let flag = return_true();
    println!("Function return_true result: {}", flag);
}

fn print_sum(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

fn add_200(num: i32) -> i32 {
    num + 200
}
fn return_true() -> bool {
    return true;
}
