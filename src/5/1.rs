use std::io;

fn main() {
    let mut graph: Vec<Vec<usize>> = Vec::new();

    graph.resize(100, Vec::new());

    let mut buf = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        if buf.trim().is_empty() {
            break;
        }

        let a: usize = buf[0..2].parse().unwrap();
        let b: usize = buf[3..5].parse().unwrap();
        graph[a].push(b);
        buf.clear();
    }

    let mut ans: i32 = 0;

    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let iter = buf.trim().split(',');

        let mut v: Vec<usize> = Vec::new();
        let mut ok = true;

        for s in iter {
            let u: usize = s.trim().parse().unwrap();

            for &x in &graph[u] {
                if v.contains(&x) {
                    ok = false;
                    break;
                }
            }

            v.push(u);
        }

        if ok {
            let n = v.len();
            ans += v[n / 2] as i32;
        }

        buf.clear();
        v.clear();
    }

    println!("{}", ans);
}
