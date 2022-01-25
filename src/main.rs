mod learning;

use std::cmp::Ordering;
use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};
use rand::{Rng, thread_rng};

fn main() {
    // hello_world();
    // guess_number();

    let s1 = String::from("123");
    let s2 = "456";
    let s3 = str_add(s1, s2);
    // println!("{}", s1);
    println!("{} {}", s2, s3);

    learning::main()
}

fn hello_world() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn guess_number() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn str_add(mut str: String, add_str: &str) -> String {
    str.push_str(add_str);
    str
}
