use std::io;

fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    _ = stdin.read_line(&mut buf);

    buf = String::new();
    _ = stdin.read_line(&mut buf);
    let buf = buf;

    let nums: Vec<i64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut under0: u64 = 0;

    for num in nums {
        if num < 0 {
            under0 += 1;
        }
    }

    let under0 = under0;

    println!("{}", under0);
}
