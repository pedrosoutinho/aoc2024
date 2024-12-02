use std::io;

fn main() {
    let mut ans: i32 = 0;

    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let iter = line.split_whitespace();

        let mut v = Vec::<i32>::new();
        for x in iter {
            v.push(x.parse().unwrap());
        }

        for j in 0..v.len() {
            let mut v2 = Vec::<i32>::new();

            for (i, x) in v.iter().enumerate() {
                if i == j {
                    continue;
                }

                v2.push(*x);
            }

            let mut f1: bool = true;
            let mut f2: bool = true;

            let mut prev: i32 = v2[0];
            for it in v2.iter().skip(1) {
                let x = *it;
                f1 &= (prev < x) & (prev + 3 >= x);
                f2 &= (prev > x) & (prev <= x + 3);
                prev = x;
            }

            if f1 || f2 {
                ans += 1;
                break;
            }
        }

        line.clear();
    }

    println!("{}", ans);
}
