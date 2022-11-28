//Vectors are resizable Vector

use std::mem;

pub fn vectors() {
    println!("using vectors in rust");

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // print!("{:?}", number);

    //you can  change on to vector
    numbers[2] = 20;
    // you cann add to vector
    numbers.push(6);
    numbers.push(7);
    numbers.pop();

    println!("number is now {:?}", numbers);

    print!("the first number on list {}", numbers[0]);

    //Vector length
    println!("Vector length {}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[1..4];
    println!("slice: {:?}", slice);

    //Loop through vector vals
    for val in numbers.iter() {
        println!("Number: {}", val);
    }
    //loop and mutate vals
    for val in numbers.iter_mut() {
        *val *= 2;
    }
    println!("New value: {:?}", numbers);
}
