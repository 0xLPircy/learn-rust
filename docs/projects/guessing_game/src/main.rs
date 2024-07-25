use std::io; //std lib ka io function imported
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
loop{
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin() //io function ka stdin use kiya
        .read_line(&mut guess) //stdin ka func read_line
        //note: this appends, not rewrites
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
// trim to remove whtespacew, u32 is us telling which num type we want
// parse returns a result type too which can be caught by expect
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("Too big!!"),
        Ordering::Equal => {
            println!("You Win!");
    break;}        
    }
}
}