use std::io;
use std::collections::VecDeque;

fn main() {
    let mut buf: String = String::new();
    let mut grid: Vec<Vec<i32>> = Vec::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }
        grid.push(
            buf.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
        buf.clear();
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut ans: i32 = 0;
    for (i, lin) in grid.iter().enumerate() {
        for (j, val) in lin.iter().enumerate() {
            if *val == 0 {
                let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                q.push_back((i, j));
                let dx = [0, 0, 1, -1];
                let dy = [1, -1, 0, 0];
                let isin = |x: i32, y: i32| x >= 0 && x < n as i32 && y >= 0 && y < m as i32;
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    if grid[x][y] == 9 {
                        ans += 1;
                    }
                    for k in 0..4 {
                        let nx = x as i32 + dx[k];
                        let ny = y as i32 + dy[k];
                        if isin(nx, ny) && grid[nx as usize][ny as usize] - grid[x][y] == 1 {
                            q.push_back((nx as usize, ny as usize));
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
