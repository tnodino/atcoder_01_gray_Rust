// https://atcoder.jp/contests/abc228/tasks/abc228_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut P = Vec::new();
    for i in 0..N {
        input! {
            P1: usize,
            P2: usize,
            P3: usize,
        }
        P.push((i, P1 + P2 + P3));
    }
    P.sort_by(|a, b| b.1.cmp(&a.1));
    let mut ans = vec!["Yes"; N];
    let M = P[K-1].1;
    for i in 0..N {
        if P[i].1 + 300 < M {
            ans[P[i].0] = "No";
        }
    }
    for a in ans {
        println!("{}", a);
    }
}