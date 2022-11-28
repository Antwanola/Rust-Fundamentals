// Variable hold preferences to data
//Variables are immutable by default
//Rust is a blocked scoped language

pub fn declare_name() {
    let name = "Antwan";
    let mut age = 30;
    println!("my name is {} and i am {} years old", name, age);
    age = 40;
    println!("my name is {} and i am {} years old", name, age);

    //Define const

    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("brad", 30);
    println!("{} is {}", my_name, my_age);
}
