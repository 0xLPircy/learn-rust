struct Point<X1, Y1>{
    x: X1,
    y: Y1
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point{
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
     let p1 = Point {x: 5, y: 10.4};
     let p2 = Point {x: "hello", y: "c"};

    //  enchanges even tho generics and now x and y are of not same type, 
    // wont work if while defining we had said T for both need to be X1 and Y1
     let p3 = p1.mixup(p2);

     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

