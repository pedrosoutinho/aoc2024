use std::io;

fn main() {
    let mut ans: i64 = 0;

    let mut buf = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let mut nums: Vec<String> = buf.split_whitespace().map(|s| s.to_string()).collect();
        nums[0].pop();
        let res: i64 = nums[0].parse().unwrap();

        let mut ope: Vec<i64> = Vec::new();
        for i in nums.iter().skip(1) {
            ope.push(i.parse().unwrap());
        }

        fn dfs(ope: &[i64], res: i64, i: usize, cur: i64) -> bool {
            let n = ope.len();
            if i == n {
                return cur == res;
            }

            if cur > res {
                return false;
            }

            let concat = |x: i64, y: i64| {
                let mut x = x;
                let mut ny = y;
                while ny > 0 {
                    x *= 10;
                    ny /= 10;
                }
                x + y
            };

            let mut ans = false;

            ans |= dfs(ope, res, i + 1, cur + ope[i]);
            ans |= dfs(ope, res, i + 1, cur * ope[i]);
            ans |= dfs(ope, res, i + 1, concat(cur, ope[i]));

            ans
        }

        if dfs(&ope, res, 1, ope[0]) {
            ans += res;
        }

        buf.clear();
    }

    println!("{}", ans);
}
