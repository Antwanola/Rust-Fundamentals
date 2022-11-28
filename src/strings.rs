//strings are of two types here. Growable(let me = String::from("antwan")) strings and immutable(let me = "antwan")

pub fn set_strings() {
    println!("I was in strings but now i am free \u{1F600}")
}

pub fn strings() {
    let mut me = String::from("antwan");
    //pushing to string here is of two types. push for char("a") or push string for all.
    // me.push('o');
    //push multi str
    me.push_str(" ola");
    //get length just like in js
    println!("getLength: {}", me.len());
    println!("string Heap: {}", me);
    //Check if empty
    println!("is the string empty? {}", me.is_empty());

    //Check if contains
    println!("Does it contain the word 'ola'? {}", me.contains("ola"));
    //Replace a word in str
    println!(
        "Replace the word 'ola' with 'olatunji' {}",
        me.replace("ola", " Olatunji")
    );
    //loop through string by whitespace

    for word in me.split_whitespace() {
        println!("{}", word);
    }
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion test
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}
