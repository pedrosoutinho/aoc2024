use std::collections::HashMap;
use std::io;

fn main() {
    let mut buf: String = String::new();
    let mut cnt: HashMap<i64, i64> = HashMap::new();
    while let Ok(n) = io::stdin().read_line(&mut buf) {
        if n == 0 {
            break;
        }

        for x in buf.split_whitespace().map(|x| x.parse::<i64>().unwrap()) {
            *cnt.entry(x).or_insert(0) += 1;
        }

        buf.clear();
    }

    let pot10: Vec<i64> = (0..19).map(|x| 10i64.pow(x)).collect();

    // pt1: 25, pt2: 75
    for _ in 0..75 {
        let mut ncnt: HashMap<i64, i64> = HashMap::new();
        for (k, v) in cnt.iter() {
            let l = k.to_string().len();
            if *k == 0 {
                *ncnt.entry(1).or_insert(0) += *v;
            } else if l % 2 == 0 {
                let x = *k / pot10[l / 2];
                let y = *k % pot10[l / 2];
                *ncnt.entry(x).or_insert(0) += *v;
                *ncnt.entry(y).or_insert(0) += *v;
            } else {
                *ncnt.entry((*k) * 2024).or_insert(0) += *v;
            }
        }

        cnt = ncnt;
    }

    let mut ans: i64 = 0;
    for (_, v) in cnt.iter() {
        ans += v;
    }
    println!("{}", ans);
}
