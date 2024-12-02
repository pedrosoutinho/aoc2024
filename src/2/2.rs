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

        for j in 0..v.len() {
            let mut v2 = Vec::<i32>::new();

            for i in 0..v.len() {
                if i != j {
                    v2.push(v[i]);
                }
            }

            let mut f1 : bool = true;
            let mut f2 : bool = true;

            let mut prev : i32 = v2[0];
            for i in 1..v2.len() {
                f1 &=  (prev + 1 <= v2[i]) & (prev + 3 >= v2[i]);
                f2 &= (prev >= v2[i] + 1) & (prev <= v2[i] + 3);
                prev = v2[i];
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
