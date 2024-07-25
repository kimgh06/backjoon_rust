use std::io::stdin;

fn main() {
    // Your code goes here

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let cnt: i128 = buffer.trim().parse().unwrap();
    println!("{}", cnt * 4);
}
