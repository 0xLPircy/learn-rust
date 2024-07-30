use std::io; 

fn main() {
    let mut far = String::new();
    println!("Enter Far:");
    io::stdin() 
        .read_line(&mut far) 
        .expect("Failed to read line"); 


    // let far: f32 = far.trim().parse().expect("Please type a number!");
    // OR
    let far: f32 = match far.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Please enter a valid number!");
            return;
        }
    };

    let cel = {
        (5.0/9.0)*(far-32.0)
    };//auto gives float to cel
    println!("The Cel is {cel}")
}
