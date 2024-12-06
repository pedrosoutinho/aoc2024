use std::io;

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut buf = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        if buf.trim().is_empty() {
            break;
        }

        grid.push(buf.trim().chars().collect());
        buf.clear();
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut s: (usize, usize) = (0, 0);

    for (i, lin) in grid.iter().enumerate() {
        for (j, _) in lin.iter().enumerate() {
            if grid[i][j] == '^' {
                s = (i, j);
            }
        }
    }

    let mut vis = vec![vec![false; m]; n];

    fn dfs(grid: &Vec<Vec<char>>, xy: (usize, usize), vis: &mut Vec<Vec<bool>>, dir: usize) {
        let (x, y) = xy;
        let n: i32 = grid.len() as i32;
        let m: i32 = grid[0].len() as i32;
        vis[x][y] = true;

        let dy = [0, 1, 0, -1];
        let dx = [-1, 0, 1, 0];

        let nx: i32 = x as i32 + dx[dir];
        let ny: i32 = y as i32 + dy[dir];

        if nx >= 0 && nx < n && ny >= 0 && ny < m {
            let nx = nx as usize;
            let ny = ny as usize;

            if grid[nx][ny] == '#' {
                dfs(grid, (x, y), vis, (dir + 1) % 4);
            }
            else {
                dfs(grid, (nx, ny), vis, dir);
            }
        }
    }

    dfs(&grid, s, &mut vis, 0);

    let mut ans = 0;
    for (i, lin) in grid.iter().enumerate() {
        for (j, _) in lin.iter().enumerate() {
            if vis[i][j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
