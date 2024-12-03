use std::io;

fn main() {
    let mut ans: i64 = 0;
    let mut e: bool = true;
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }

        for i in 0..line.len() {
            if line[i..].starts_with("do()") {
                e = true;
            }

            if line[i..].starts_with("don't()") {
                e = false;
            }

            if line[i..].starts_with("mul(") && e {
                let mut s: String = String::new();
                let mut r: String = String::new();
                let mut f: i32 = 0;
                let mut ok: bool = true;

                for c in line[i + 4..].chars() {
                    if c == ',' {
                        f += 1;
                        if f > 1 || s.is_empty() {
                            ok = false;
                            break;
                        }
                        continue;
                    }

                    if c == ')' {
                        if f == 0 || r.is_empty() {
                            ok = false;
                        }
                        break;
                    }

                    if c.is_ascii_digit() {
                        if f == 0 {
                            s.push(c);
                        } else {
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
