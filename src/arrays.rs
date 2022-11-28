//Arrays are fixed list on here where elements can only be the same data type
use std::mem;

pub fn arrays() {
    println!("just arrays mehn");

    let mut numbers = [1, 2, 3, 4, 5];
    // print!("{:?}", number);

    //you can only add on to array but not change what the array is --> we can change index 3 to 20 but not the shape of the array.
    numbers[2] = 20;
    println!("number is now {:?}", numbers);

    print!("the first number on list {}", numbers[0]);

    //array length
    println!("array length {}", numbers.len());

    //Arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[1..4];
    println!("slice: {:?}", slice);
}
