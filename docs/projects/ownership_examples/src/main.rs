fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {s1}, s2 = {s2}");
    let s = String::from("hello");  // s comes into scope
        takes_ownership(s);    
        let reference_to_nothing = dangle();//ERR
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

  //ERRRR
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
