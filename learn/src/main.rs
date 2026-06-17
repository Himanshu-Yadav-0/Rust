fn main(){
    let greetings: String = String::from("hello Sir");
    println!("{}", greetings);
    let char: Option<char> = greetings.chars().nth(1);
    match char {
        Some(c) => println!("Char: {}", c),
        None => println!("No char at that index"),
    }
}