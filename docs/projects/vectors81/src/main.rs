fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // let mut v2 = Vec::new();

    // v2.push(5);
    // v2.push(6);
    // v2.push(7);
    // v2.push(8);

    // let mut v = vec![1, 2, 3, 4, 5]; // ! is something about generics

    // let third: &i32 = &v[2]; //same as c
    // println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2); //ew but okay
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    // // ITERATE
    // for i in &mut v{
    //     *i +=50; //so *i gives orignal value location mutable
    // }
    // for i in &v {
    //     println!("{i}");
    
    // } 

    // ERRR SINCE MUT AND IMMUTE BOTH TAKEN
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // IMMUTE
    // v.push(6); //MUT errr
    // println!("The first element is: {first}");  

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    } 

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}
