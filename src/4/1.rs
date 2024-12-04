use std::io;

fn line(grid: &[String], x: usize) -> String {
    grid[x].to_string()
}

fn row(grid: &[String], y: usize) -> String {
    let mut s = String::new();

    for lin in grid {
        s.push(lin.chars().nth(y).unwrap());
    }

    s
}

fn diag(grid: &[String], x: usize, y: usize) -> String {
    let n = grid.len();
    let m = grid[0].len();

    let mut s = String::new();

    let mut i = x;
    let mut j = y;

    while i < n && j < m {
        s.push(grid[i].chars().nth(j).unwrap());
        i += 1;
        j += 1;
    }

    s
}

fn diag2(grid: &[String], x: usize, y: usize) -> String {
    let m = grid[0].len();

    let mut s = String::new();

    let mut i = x;
    let mut j = y;

    while j < m {
        s.push(grid[i].chars().nth(j).unwrap());
        if i == 0 {
            break;
        }
        i -= 1;
        j += 1;
    }

    s
}

fn count(str: &String) -> i32 {
    let mut count = 0;
    let mut s = str.to_string();
    for _ in 0..2 {
        for i in 0..s.len() {
            if s[i..].starts_with("XMAS") {
                count += 1;
            }
        }

        s = s.chars().rev().collect();
    }

    count
}

fn main() {
    let mut grid: Vec<String> = Vec::new();

    let mut buf = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        grid.push(buf.trim().to_string());

        buf.clear();
    }

    let mut ans: i32 = 0;

    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        ans += count(&line(&grid, i));
        ans += count(&diag(&grid, i, 0));
        ans += count(&diag2(&grid, i, 0));
    }

    for i in 0..m {
        ans += count(&row(&grid, i));
        if i > 0 {
            ans += count(&diag(&grid, 0, i));
            ans += count(&diag2(&grid, n - 1, i));
        }
    }

    println!("{}", ans);
}
