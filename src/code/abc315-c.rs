// https://atcoder.jp/contests/abc315/tasks/abc315_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = vec![Vec::new(); N];
    for _ in 0..N {
        input! {
            F: usize,
            S: usize,
        }
        vec[F-1].push(S);
    }
    let mut ma = Vec::new();
    let mut ans = 0;
    for i in 0..N {
        if vec[i].len() == 0 {
            continue;
        }
        vec[i].sort_by(|a, b| b.cmp(&a));
        ma.push(vec[i][0]);
        if vec[i].len() == 1 {
            continue;
        }
        ans = max(ans, vec[i][0] + vec[i][1] / 2);
    }
    if ma.len() >= 2 {
        ma.sort_by(|a, b| b.cmp(&a));
        ans = max(ans, ma[0] + ma[1]);
    }
    println!("{}", ans);
}