// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function(x: i32) {
//     println!("Another function.");
// }
fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    let sum = sum(10,2);
    println!("The value of x from expression is: {x}");
    println!("Sum is {sum}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
    5
}
fn sum(a:i32, b:i32) -> i32{
    a+b
}