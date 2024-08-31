pub fn run() {
    // Print to console
    println!("hello print.rs file here!");

    // basic format
    println!("{} to many {} placeholder, of diff types{}", 1, 2, "SEE");

    // position changes
    println!("{0} is a type of lang, {1} is also a type sof lang, {0} is also subs, and {1} is also a letter", "Rust", "C");

    // names
    println!("my name is {name}, age is {age}", age =22, name = "esha");

    // placeholder traits
    println!("Bin: {:b}, Hex:{:x}, Oct: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10+3 ={}", 10+3);
}