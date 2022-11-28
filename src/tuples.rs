pub fn tuples() {
    let person: (&str, &str, i8) = ("adam", "Antwan", 45);
    print!("lets see what person looks like {:#?}", person);
    print!("{} is from {} and is {}", person.1, person.0, person.2)
}
