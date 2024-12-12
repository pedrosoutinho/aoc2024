use std::collections::VecDeque;
use std::io;

fn main() {
    let mut buf: String = String::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }
        grid.push(buf.trim().chars().collect());
        buf.clear();
    }

    let n = grid.len();
    let m = grid[0].len();
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let isin = |x: i32, y: i32| x >= 0 && x < n as i32 && y >= 0 && y < m as i32;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut group = vec![vec![0; m]; n];
    let mut cur = 1;
    let mut tots: Vec<usize> = Vec::new();
    tots.push(0);
    for (i, lin) in grid.iter().enumerate() {
        for (j, _) in lin.iter().enumerate() {
            if group[i][j] != 0 {
                continue;
            }
            let mut cnt = [0, 0, 0, 0, 0];
            let mut tot = 0;
            q.push_back((i, j));
            group[i][j] = cur;
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                let mut tmp = 0;
                tot += 1;
                for k in 0..4 {
                    let nx = x as i32 + dx[k];
                    let ny = y as i32 + dy[k];
                    if isin(nx, ny) && grid[nx as usize][ny as usize] == grid[i][j] {
                        tmp += 1;
                        if group[nx as usize][ny as usize] == 0 {
                            q.push_back((nx as usize, ny as usize));
                            group[nx as usize][ny as usize] = cur;
                        }
                    }
                }
                cnt[tmp] += 1;
            }

            cur += 1;
            tots.push(tot);
        }
    }

    let mut ans = 0;
    for (g, tot) in tots.iter().enumerate().skip(1) {
        let mut sides = 0;
        let mut fu = 1;
        let mut fd = 1;
        for i in 0..n {
            for j in 0..m {
                if group[i][j] == g {
                    if !isin(i as i32 - 1, j as i32) || group[i][j] != group[i - 1][j] {
                        if fu == 1 {
                            fu = 0;
                            sides += 1;
                        }
                    } else {
                        fu = 1;
                    }

                    if !isin(i as i32 + 1, j as i32) || group[i][j] != group[i + 1][j] {
                        if fd == 1 {
                            fd = 0;
                            sides += 1;
                        }
                    } else {
                        fd = 1;
                    }
                } else {
                    fu = 1;
                    fd = 1;
                }
            }
        }
        let mut fl = 1;
        let mut fr = 1;
        for j in 0..m {
            for i in 0..n {
                if group[i][j] == g {
                    if !isin(i as i32, j as i32 - 1) || group[i][j] != group[i][j - 1] {
                        if fl == 1 {
                            fl = 0;
                            sides += 1;
                        }
                    } else {
                        fl = 1;
                    }

                    if !isin(i as i32, j as i32 + 1) || group[i][j] != group[i][j + 1] {
                        if fr == 1 {
                            fr = 0;
                            sides += 1;
                        }
                    } else {
                        fr = 1;
                    }
                } else {
                    fl = 1;
                    fr = 1;
                }
            }
        }

        ans += tot * sides;
    }

    println!("{}", ans);
}
