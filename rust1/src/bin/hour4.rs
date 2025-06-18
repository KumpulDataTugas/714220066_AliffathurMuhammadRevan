fn main() {
    // If Statement
    println!("If Statement");
    let num = 10;
    if num == 10 {
        println!("num is equal to 10");
    }
    println!(" ");

    // If-Else Statement
    println!("If-Else Statement");
    let x = 100;
    let y = 200;
    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is smaller than y");
    }
    println!(" ");

    // Let-If Statement
    println!("Let-If Statement");
    let num = if true {
        100
    } else {
        200
    };
    println!("The value of num is {}", num);
    println!(" ");

    // Loop - Break Statement
    println!("Loop - Break Statement");
    let mut num = 5;
    loop {
        println!("C# in {} Hours", num);
        if num == 8 {
            break;
        }
        num += 1;
    }
    println!(" ");

    // For Statement
    println!("For Statement");
    for num in 5..9 {
        println!("Java in {} Hours", num);
    }
    println!(" ");

    // While Statement
    println!("While Statement");
    let mut num = 0;
    while num <= 8 {
        print!("{} ", num);
        num += 1;
    }
    println!(" ");

    // Tuples
    println!("Tuples");
    let t = ("Python in", 8, "Hours", true);
    println!("{} {} {} {}", t.0, t.1, t.2, t.3);
    println!(" ");

    // Match Statement
    println!("Match Statement");
    let num: i32 = 3;
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("something else"),
    }
}
