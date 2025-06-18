struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

trait Calculate {
    fn area(&self) -> f32;
}

impl Calculate for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

pub trait Show {
    fn show(&self);
}

impl<T> Show for T
where
    T: ToString,
{
    fn show(&self) {
        print!("{}", self.to_string());
    }
}

struct Game {
    number: i32,
}

impl Drop for Game {
    fn drop(&mut self) {
        println!("The #{} Winner.", self.number);
    }
}

fn closure_example_1() {
    let my_closure = |num: i32| num + 200;
    let num = 100;
    println!("{}", my_closure(num));
}

fn closure_example_2() {
    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut my_closure = |c: char| {
            capacity.push(c);
        };
        my_closure('G');
    }
    println!("{:?}", capacity);
}

fn main() {
    let obj = Circle { radius: 2000.00 };
    println!("The Circle area (method) is: {}", obj.area());
    println!("The Circle area (trait) is: {}", Calculate::area(&obj));
    String::from("C# in 8 Hours").show();

    let _baseball = Game { number: 3 };
    let _football = Game { number: 2 };
    let _basketball = Game { number: 1 };

    closure_example_1();
    closure_example_2();
}
