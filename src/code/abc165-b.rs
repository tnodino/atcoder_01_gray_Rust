// https://atcoder.jp/contests/abc165/tasks/abc165_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let mut now = 100;
    let mut ans = 0;
    while now < X {
        now += now / 100;
        ans += 1;
    }
    println!("{}", ans);
}