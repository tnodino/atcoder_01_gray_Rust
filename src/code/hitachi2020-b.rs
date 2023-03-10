// https://atcoder.jp/contests/hitachi2020/tasks/hitachi2020_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        M: usize,
        a: [usize; A],
        b: [usize; B],
    }
    let mut ans = a.iter().min().unwrap() + b.iter().min().unwrap();
    for _ in 0..M {
        input! {
            x: usize,
            y: usize,
            c: usize,
        }
        ans = min(ans, a[x-1] + b[y-1] - c);
    }
    println!("{}", ans);
}