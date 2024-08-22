fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list{
        if number>largest{
            largest = number;
        }
    }
    largest
}

fn main() {
    let num_list = vec![34, 50, 24, 100, 65];

    // let mut largest = &num_list[0];

    // for number in &num_list{
    //     if number>largest{
    //         largest = number;
    //     }
    // }
    // println!("largest is {largest}")
    let result = largest(&num_list);
    println!("largest is {result}");

}

