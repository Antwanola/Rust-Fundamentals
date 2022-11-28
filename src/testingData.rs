// mod print;
fn main() {
    // println!("Hello, world!");
    // print::run();

    println!("{}", 1);
    //Basic formatting
    println!("{} is from {}", "brad", "mars");

    //Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mars", "Code"
    );

    //Named Arguments
    println!(
        "{name} likes to {activity}",
        name = "John",
        activity = "Play"
    );

    //Placeholder traits

    println!("Binary: {:b} Hex:{:x} Octal: {:o}", 10, 10, 10);

    //PlACEHOLDER FOR DEBUG TRAIT
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);