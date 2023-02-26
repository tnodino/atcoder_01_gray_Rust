// https://atcoder.jp/contests/abc064/tasks/abc064_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    a.sort();
    println!("{}", a[N-1] - a[0]);
}