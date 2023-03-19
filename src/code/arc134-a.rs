// https://atcoder.jp/contests/arc134/tasks/arc134_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
        W: usize,
        mut a: [usize; N],
    }
    a.push(L);
    let mut r = 0;
    let mut ans = 0;
    for v in a {
        if v > r {
            ans += (v - r + W - 1) / W;
        }
        r = v + W;
    }
    println!("{}", ans);
}