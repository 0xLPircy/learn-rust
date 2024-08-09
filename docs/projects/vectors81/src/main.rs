fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // let mut v2 = Vec::new();

    // v2.push(5);
    // v2.push(6);
    // v2.push(7);
    // v2.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; //same as c
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); //ew but okay
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    // ERRR SINCE MUT AND IMMUTE BOTH TAKEN
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");   
}
