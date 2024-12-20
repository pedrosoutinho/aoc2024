use std::io;

fn main() {
    let mut ans: i64 = 0;
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }

        for i in 0..line.len() {
            if line[i..].starts_with("mul(") {
                let mut s: String = String::new();
                let mut r: String = String::new();
                let mut f: i32 = 0;
                let mut ok: bool = true;

                for c in line[i + 4..].chars() {
                    if c == ',' {
                        f += 1;
                        continue;
                    }

                    if c == ')' {
                        break;
                    }

                    if c.is_ascii_digit() {
                        if f == 0 {
                            s.push(c);
                        } else if f == 1 {
                            r.push(c);
                        }
                    } else {
                        ok = false;
                        break;
                    }
                }

                if !ok {
                    continue;
                }

                let a: i64 = s.parse().unwrap();
                let b: i64 = r.parse().unwrap();
                ans += a * b;
            }
        }

        line.clear();
    }

    println!("{}", ans);
}
