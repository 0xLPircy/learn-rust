fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {s1}, s2 = {s2}");
    let s = String::from("hello");  // s comes into scope
        takes_ownership(s);    

}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
