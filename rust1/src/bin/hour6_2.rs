// hour6_private_fn_error.rs

mod my_module {
    pub fn a() {
        println!("function a");
    }
    fn b() {
        println!("function b");
    }
}

fn main() {
    my_module::a();
    my_module::b(); // MEANT TO BE ERROR
}
