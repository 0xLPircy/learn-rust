#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let h :u32 = 10;
    // let w :u32 = 5;
    // let rec1 :(u32,u32) = (10,5); 
    let rect1 = Rectangle {
        width: 10,
        height: 3,
    };
    println!("rect1 is {rect1:?}");
    dbg!(&rect1);  
    let scale = 2;
    dbg!(30*scale);
    println!("Area is {} sq units", area(&rect1));

}

fn area (recDimen: &Rectangle) -> u32 {
    // w*h
    // recDimen.0*recDimen.1
    recDimen.width*recDimen.height
}
