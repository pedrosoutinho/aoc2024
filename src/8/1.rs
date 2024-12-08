use std::collections::HashMap;
use std::io;

fn main() {
    let mut ans: i64 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut buf = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        grid.push(buf.trim().chars().collect());
        buf.clear();
    }

    for (i, lin) in grid.iter().enumerate() {
        for (j, c) in lin.iter().enumerate() {
            if map.contains_key(c) {
                let v = map.get_mut(c).unwrap();
                v.push((i, j));
            } else {
                let v = vec![(i, j)];
                map.insert(*c, v);
            }
        }
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut sig = vec![vec![false; m]; n];

    for (&c, v) in map.iter() {
        if c == '.' {
            continue;
        }

        for (i, &ab) in v.iter().enumerate() {
            for (j, &cd) in v.iter().enumerate() {
                if i == j {
                    continue;
                }

                let (a, b) = ab;
                let (c, d) = cd;

                let dx = c as i32 - a as i32;
                let dy = d as i32 - b as i32;

                let x1 = c as i32 + dx;
                let y1 = d as i32 + dy;

                let x2 = a as i32 - dx;
                let y2 = b as i32 - dy;

                let isin = |ij: (i32, i32), n: usize, m: usize| -> bool {
                    let (i, j) = ij;
                    let n = n as i32;
                    let m = m as i32;

                    if i >= 0 && j >= 0 && i < n && j < m {
                        return true;
                    }

                    false
                };

                if isin((x1, y1), n, m) {
                    sig[x1 as usize][y1 as usize] = true;
                }

                if isin((x2, y2), n, m) {
                    sig[x2 as usize][y2 as usize] = true;
                }
            }
        }
    }

    for lin in sig.iter() {
        for &f in lin.iter() {
            if f {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
