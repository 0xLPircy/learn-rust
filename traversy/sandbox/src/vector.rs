use std::mem;
pub fn run () {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // reassign value
    numbers[2] = 20;

    // Add on to vec
    numbers.push(5);
    numbers.pop();

    println!("{:?}", numbers);

    // single print
    println!("{}", numbers[0]);

    // length
    println!("Vector len: {}", numbers.len());

    // arrays are stack alloc, can see size
    println!("Vec size: {} bytes", mem::size_of_val(&numbers));

    // slicesss
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}