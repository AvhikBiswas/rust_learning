use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Any Number from 1 to 100");
    println!("Please Input Your Number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Falid to take input");

        println!("Guess Number is {value}");
        // println!("Secrate Number is {}", secret_number);

        let value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter Valid Number");
                continue;
            }
        };

        match value.cmp(&secret_number) {
            Ordering::Less => println!("Value is Less"),
            Ordering::Greater => println!("Value is Big"),
            Ordering::Equal => {
                println!("You Guessed it right");
                break;
            }
        }
    }
}
