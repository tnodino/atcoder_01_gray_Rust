// https://atcoder.jp/contests/abc095/tasks/arc096_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        X: usize,
        Y: usize,
    }
    let mut ans: usize = !0;
    for i in 0..=max(X, Y) {
        let mut price = i * C * 2;
        if i < X {
            price += (X - i) * A;
        }
        if i < Y {
            price += (Y - i) * B;
        }
        ans = min(ans, price);
    }
    println!("{}", ans);
}