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

        let n = ope.len();

        for msk in 0..(1 << (n - 1)) {
            let mut cur = ope[0];

            for it in 0..(n - 1) {
                if (msk >> it) & 1 == 1 {
                    cur += ope[it + 1];
                } else {
                    cur *= ope[it + 1];
                }
            }

            if cur == res {
                ans += res;
                break;
            }
        }

        buf.clear();
    }

    println!("{}", ans);
}
