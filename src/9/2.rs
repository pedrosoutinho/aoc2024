use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut line = vec![];
    let mut id = 0;
    let mut space = vec![];
    let mut block = vec![];
    for i in 0..buf.len() - 1 {
        let x: usize = buf.trim().chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            block.push((line.len(), x));
            for _ in 0..x {
                line.push(id);
            }
            id += 1;
        } else {
            space.push((line.len(), x));
            line.resize(line.len() + x, 0);
        }
    }

    for a in (0..block.len()).rev() {
        let (i, x) = block[a];
        let id = line[i];

        for val in space.iter_mut() {
            let (j, y) = *val;
            if i <= j {
                continue;
            }

            if x > y {
                continue;
            }

            for val in line.iter_mut().skip(i).take(x) {
                *val = 0;
            }

            for val in line.iter_mut().skip(j).take(x) {
                *val = id;
            }

            *val = (j + x, y - x);
            break;
        }
    }

    let mut ans: i64 = 0;
    for (i, &val) in line.iter().enumerate() {
        ans += val as i64 * i as i64;
    }

    println!("{}", ans);
}
