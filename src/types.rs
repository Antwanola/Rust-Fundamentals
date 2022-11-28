pub fn types() {
    let types = "types";
    let x = 2.5;
    let y: i64 = 5718576734527534;
    let z: i32 = 846468;
    println!("this is the {} file.", types);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let is_active: bool = true;
    let is_greater: bool = 10 < 5;
    //add char
    let smile = '\u{1F600}';
    println!("{:#?}", (x, y, z, is_active, is_greater, smile));

    //get boolean from exp
}
