use std::io;

fn main() {
    println!("Welcome to Fibonacci number generator!");
    println!("Enter n of fibonacci (0 <= n <= 92)");

    loop {

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to input");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("It doesn't look like a number"); continue; }
        };

        if n >= 93 {
            println!("It's too big!");
            continue;
        } else if n < 0 {
            println!("It's too small!");
            continue;
        }

        println!("You inputted {n}");

        // The calculating
        let mut f1: u64 = 0;
        let mut f2: u64 = 1;

        for _ in 0..n {
            f2 += f1;
            f1 = f2 - f1;
        }
        println!("{f1}");

        break;
    }

    println!("Enter any key to exit");
}
