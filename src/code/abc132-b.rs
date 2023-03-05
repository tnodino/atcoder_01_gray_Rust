// https://atcoder.jp/contests/abc132/tasks/abc132_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut ans = 0;
    for i in 1..n-1 {
        if p[i-1] < p[i] && p[i] < p[i+1] {
            ans += 1;
        }
        if p[i-1] > p[i] && p[i] > p[i+1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}