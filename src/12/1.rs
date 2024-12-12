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
    let mut vis = vec![vec![false; m]; n];
    let mut ans = 0;
    for (i, lin) in grid.iter().enumerate() {
        for (j, _) in lin.iter().enumerate() {
            if vis[i][j] {
                continue;
            }
            let mut cnt = [0, 0, 0, 0, 0];
            let mut tot = 0;
            q.push_back((i, j));
            vis[i][j] = true;
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                let mut tmp = 0;
                tot += 1;
                for k in 0..4 {
                    let nx = x as i32 + dx[k];
                    let ny = y as i32 + dy[k];
                    if isin(nx, ny) && grid[nx as usize][ny as usize] == grid[i][j] {
                        tmp += 1;
                        if !vis[nx as usize][ny as usize] {
                            q.push_back((nx as usize, ny as usize));
                            vis[nx as usize][ny as usize] = true;
                        }
                    }
                }
                cnt[tmp] += 1;
            }

            ans += tot * (cnt[0] * 4 + cnt[1] * 3 + cnt[2] * 2 + cnt[3]);
        }
    }

    println!("{}", ans);
}
