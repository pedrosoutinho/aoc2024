use std::io;

fn main() {
    let mut ans : i32 = 0;

    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let mut iter = line.split_whitespace();

        let mut v = Vec::<i32>::new();
        while let Some(x) = iter.next() {
            v.push(x.parse().unwrap());
        }

        let mut prev = v[0];
        let mut f1 : bool = true;
        let mut f2 : bool = true;
        for i in 1..v.len() {
            f1 &= (prev + 1 <= v[i]) & (prev + 3 >= v[i]);
            f2 &= (prev >= v[i] + 1) & (prev <= v[i] + 3);
            prev = v[i];
        }

        if f1 || f2 {
            ans += 1;
        }

        line.clear();
    }

    println!("{}", ans);
}
