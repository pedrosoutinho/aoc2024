use std::io;

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut buf = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        grid.push(buf.trim().chars().collect());

        buf.clear();
    }

    let mut ans: i32 = 0;

    let n = grid.len();
    let m = grid[0].len();

    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if grid[i][j] == 'A' {
                let a = grid[i - 1][j - 1];
                let b = grid[i + 1][j + 1];
                let c = grid[i - 1][j + 1];
                let d = grid[i + 1][j - 1];

                let f1 = (a == 'M' && b == 'S') || (a == 'S' && b == 'M');
                let f2 = (c == 'M' && d == 'S') || (c == 'S' && d == 'M');
                if f1 && f2 {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
