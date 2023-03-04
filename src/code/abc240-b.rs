// https://atcoder.jp/contests/abc240/tasks/abc240_b

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
    a.dedup();
    println!("{}", a.len());
}