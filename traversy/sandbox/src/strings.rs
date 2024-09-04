pub fn run() {
    let mut hello = String::from("hello ");

    // GetLength
    println!("Length: {}", hello.len());

    // push char
    hello.push('W');

    // push string
    hello.push_str("o rld");

    // capacity
    println!("Capacuty: {}", hello.capacity());

    // if empty
    println!("is empty: {}", hello.is_empty());

    // containes
    println!("contain 'world': {}", hello.contains("World"));

    // replace
    println!("replace: {}", hello.replace("world", "there"));

    // loop through
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // create string with perticular capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("s: {}", s);

    // assert to test, PANIC if not and stop running
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("FULL: {}", hello);
}