use std::mem;
pub fn run () {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // single print
    println!("{}", numbers[0]);

    // length
    println!("Array len: {}", numbers.len());

    // arrays are stack alloc, can see size
    println!("Arr size: {} bytes", mem::size_of_val(&numbers));

    // slicesss
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}