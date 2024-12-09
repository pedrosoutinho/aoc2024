use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut line = vec![];
    let mut id = 0;
    for i in 0..buf.len() - 1 {
        let x: usize = buf.trim().chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            for _ in 0..x {
                line.push(id);
            }
            id += 1;
        } else {
            for _ in 0..x {
                line.push(-1);
            }
        }
    }

    for i in 0..line.len() {
        if i >= line.len() {
            break;
        }

        if line[i] != -1 {
            continue;
        }

        loop {
            let x = line.pop().unwrap();
            if x == -1 {
                continue;
            }
            if i < line.len() {
                line[i] = x;
            }

            break;
        }
    }

    let mut ans: i64 = 0;
    for (i, &val) in line.iter().enumerate() {
        ans += val as i64 * i as i64;
    }

    println!("{}", ans);
}
