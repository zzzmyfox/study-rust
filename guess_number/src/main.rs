use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess a number between 0-100");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("input unexpection");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a valid number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
