// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn contain(&self, rec2:&Rectangle) -> bool{
        self.width<rec2.width && self.height<rec2.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let h :u32 = 10;
    // let w :u32 = 5;
    // let rec1 :(u32,u32) = (10,5); 
    let sq = Rectangle::square(3);
    let rect1 = Rectangle {
        width: 10,
        height: 3,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 6,
    };
    println!("area is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.contain(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.contain(&rect1));    // println!("rect1 is {rect1:?}");
    // dbg!(&rect1);  
    // let scale = 2;
    // dbg!(30*scale);
    // println!("Area is {} sq units", area(&rect1));

}

// fn area (recDimen: &Rectangle) -> u32 {
//     // w*h
//     // recDimen.0*recDimen.1
//     recDimen.width*recDimen.height
// }
