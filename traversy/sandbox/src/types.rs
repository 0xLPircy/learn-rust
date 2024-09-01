pub fn run(){
    // default compiler assumes
    let x = 1;
    let y = 2.3;

    // set explicitly
    let z:i64 = 123456789;

    // max 
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    // bool
    let is_smaller = 10<5;

    // char (can do ANY unicode even emojies)
    let emo = "\u{1F600}";

    println!("{:?}", (x, y, z, is_smaller, emo));
} 