use std::io;

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
                let mut vis = vec![vec![false; m]; n];

                fn dfs(grid: &Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, x: i32, y: i32) -> i32 {
                    let dx = [0, 0, 1, -1];
                    let dy = [1, -1, 0, 0];
                    let isin = |x: i32, y: i32| -> bool {
                        x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
                    };

                    let mut res = 0;
                    vis[x as usize][y as usize] = true;

                    if grid[x as usize][y as usize] == 9 {
                        res += 1;
                    }

                    for i in 0..4 {
                        let nx = x + dx[i];
                        let ny = y + dy[i];
                        if isin(nx, ny)
                            && grid[nx as usize][ny as usize] - grid[x as usize][y as usize] == 1
                            && !vis[nx as usize][ny as usize]
                        {
                            res += dfs(grid, vis, nx, ny);
                        }
                    }

                    res
                }

                ans += dfs(&grid, &mut vis, i as i32, j as i32);
            }
        }
    }

    println!("{}", ans);
}
