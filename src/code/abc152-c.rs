// https://atcoder.jp/contests/abc152/tasks/abc152_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut mi = N + 1;
    let mut ans = 0;
    for p in P {
        if mi > p {
            ans += 1;
            mi = p;
        }
    }
    println!("{}", ans);
}