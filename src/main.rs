use std::io;
use std::collections::HashMap;

fn main() {
    let mut v1 = Vec::<i32>::new();
    let mut v2 = Vec::<i32>::new();

    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        v1.push(a);
        v2.push(b);
        line.clear();
    }

    let mut cnt: HashMap<i32, i32> = HashMap::new();

    for i in 0..v2.len() {
        let k = &v2[i];
        if cnt.contains_key(k) {
            let c = cnt.get_mut(k).unwrap();
            *c += 1;
        }
        else {
            cnt.insert(*k, 1);
        }
    }

    let mut ans : i64 = 0;

    for i in 0..v1.len() {
        let k = &v1[i];
        if cnt.contains_key(k) {
            let c = cnt.get(k).unwrap();
            ans += (*c * k) as i64;
        }
    }

    println!("{}", ans);
}
