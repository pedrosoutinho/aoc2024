use std::io;

fn main() {
    let mut v1 = Vec::<i32>::new();
    let mut v2 = Vec::<i32>::new();

    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        v1.push(a);
        v2.push(b);
        line.clear();
    }

    v1.sort();
    v2.sort();

    let mut ans : i64 = 0;

    for i in 0..v1.len() {
        ans += (v1[i] - v2[i]).abs() as i64;
    }

    println!("{}", ans);
}
