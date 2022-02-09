use std::io;

fn main() {
    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");
    let celsius = celsius.trim().parse::<i32>().expect("Failed parsing int");
    let farenheit = (celsius * 9 / 5) + 32;
    println!("{}", farenheit);
}