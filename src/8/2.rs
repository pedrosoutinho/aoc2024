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
                map.insert(*c, vec![(i, j)]);
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

                let mut dx = c as i32 - a as i32;
                let mut dy = d as i32 - b as i32;

                fn gcd(a: i32, b: i32) -> i32 {
                    if b == 0 {
                        return a;
                    }

                    gcd(b, a % b)
                }

                let g = gcd(dx.abs(), dy.abs());
                dx /= g;
                dy /= g;

                let isin = |ij: (i32, i32), n: usize, m: usize| -> bool {
                    let (i, j) = ij;
                    let n = n as i32;
                    let m = m as i32;

                    if i >= 0 && j >= 0 && i < n && j < m {
                        return true;
                    }

                    false
                };

                for _ in 0..2 {
                    let mut x = a as i32;
                    let mut y = b as i32;

                    while isin((x, y), n, m) {
                        sig[x as usize][y as usize] = true;
                        x += dx;
                        y += dy;
                    }

                    dx *= -1;
                    dy *= -1;
                }
            }
        }
    }

    for lin in sig {
        for flag in lin {
            if flag {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
