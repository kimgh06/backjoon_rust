use std::io::stdin;

fn main() {
    // Your code goes here
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let numbers: Vec<i32> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n: i32 = numbers[0];
    let mut k: i32 = numbers[1];
    let mut coins = Vec::new();
    for _ in 0..n {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let coin: i32 = buffer.trim().parse().unwrap();
        coins.push(coin);
    }
    coins.sort_by(|a, b| b.cmp(a));
    let mut cnt = 0;
    for coin in coins {
        cnt += k / coin;
        k %= coin;
    }
    println!("{}", cnt);
}
