fn example7_1() {
    let v = vec![100, 200, 300, 400];
    println!("First element is :{}", v[0]);
    println!("Second element is :{}", v[1]);
    println!("Third element is :{}", v[2]);
    println!("Fourth element is :{}", v[3]);
}

fn example7_2() {
    let v = vec![8; 3];
    println!("First element is :{}", v[0]);
    println!("Second element is :{}", v[1]);
    println!("Third element is :{}", v[2]);
}

fn example7_3() {
    let mut v = Vec::new();
    v.push('R');
    v.push('U');
    v.push('B');
    v.push('Y');
    for n in v {
        print!("{}", n);
    }
    println!();
}

fn example7_4() {
    let num = 3;
    match num {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        _ => println!("others"),
    }
}

fn example7_5() {
    let x = 3;
    match x {
        2..=6 => println!("from 2 to 6"),
        _ => println!("others"),
    }
}

fn example7_6() {
    let x = 5;
    match x {
        var @ 2..=6 => println!("{}", var),
        _ => println!("others"),
    }
}

fn example7_7() {
    let x: Option<bool> = Some(true);
    let y: Option<i32> = Some(10);
    let z: Option<f64> = Some(20.88);
    let n: Option<i32> = None;

    match x {
        Some(x) => println!("x = {}", x),
        None => println!("x = None"),
    }
    match y {
        Some(y) => println!("y = {}", y),
        None => println!("y = None"),
    }
    match z {
        Some(z) => println!("z = {}", z),
        None => println!("z = None"),
    }
    match n {
        Some(n) => println!("n = {}", n),
        None => println!("n = None"),
    }
}

fn main() {
    example7_1();
    example7_2();
    example7_3();
    example7_4();
    example7_5();
    example7_6();
    example7_7();
}
