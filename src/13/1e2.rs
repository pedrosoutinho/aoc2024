use std::io;

fn main() {
    let mut buf: String = String::new();
    let mut ans: i64 = 0;
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let mut dx1 = 0;
        let mut tmp = String::new();
        for c in buf.trim().chars() {
            if c.is_ascii_digit() {
                tmp.push(c);
            } else if c == ',' {
                dx1 = tmp.parse::<i64>().unwrap();
                tmp.clear();
            }
        }
        let dy1 = tmp.parse::<i64>().unwrap();
        tmp.clear();

        let mut dx2 = 0;
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        for c in buf.trim().chars() {
            if c.is_ascii_digit() {
                tmp.push(c);
            } else if c == ',' {
                dx2 = tmp.parse::<i64>().unwrap();
                tmp.clear();
            }
        }
        let dy2 = tmp.parse::<i64>().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let mut px = 0;
        let mut tmp = String::new();
        for c in buf.trim().chars() {
            if c.is_ascii_digit() {
                tmp.push(c);
            } else if c == ',' {
                px = tmp.parse::<i64>().unwrap();
                tmp.clear();
            }
        }
        let mut py = tmp.parse::<i64>().unwrap();
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        buf.clear();

        let det = dx1 * dy2 - dx2 * dy1;
        if det == 0 {
            continue;
        }

        // pt2
        let shift = 10000000000000;
        px += shift;
        py += shift;

        let mut i = dy2 * px - py * dx2;
        let mut j = -dy1 * px + py * dx1;
        if (i % det != 0) || (j % det != 0) {
            continue;
        }

        i /= det;
        j /= det;
        if i < 0 || j < 0 {
            continue;
        }

        ans += 3 * i + j;
    }

    println!("{}", ans);
}
