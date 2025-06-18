// hour6_combined.rs

// Example 6.1: Simple module
mod my_module {
    pub fn test() {
        println!("Hello My Friends!");
    }
}

fn hour6_example1() {
    my_module::test();
}

// Example 6.2: Embedded module
mod m1 {
    pub fn a() {
        println!("m1 module");
    }
    pub mod m2 {
        pub fn b() {
            println!("m2 module");
        }
    }
}

fn hour6_example2() {
    m1::a();
    m1::m2::b();
}

// Example 6.4: External file module (simulated here inline)
mod ex_file {
    pub fn ex_fun() {
        println!("{}", "External Text");
    }
}

fn hour6_example3() {
    use ex_file::ex_fun;
    ex_fun();
}

// Example 6.6: Private function called internally
mod my_module2 {
    pub fn a() {
        println!("function a");
        b(); // OK, called internally
    }
    fn b() {
        println!("function b");
    }
}

fn hour6_example4() {
    my_module2::a();
}

// Example 6.7: Using super to call parent function
mod sup_module {
    fn a() -> i32 {
        100
    }

    pub mod sub_module {
        use super::a;
        pub fn b() {
            println!("{}", a());
        }
    }
}

fn hour6_example5() {
    sup_module::sub_module::b();
}

fn main() {
    println!("--- Example 6.1 ---");
    hour6_example1();

    println!("--- Example 6.2 ---");
    hour6_example2();

    println!("--- Example 6.4 ---");
    hour6_example3();

    println!("--- Example 6.6 ---");
    hour6_example4();

    println!("--- Example 6.7 ---");
    hour6_example5();
}
