// https://atcoder.jp/contests/abc046/tasks/abc046_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut vec = vec![a, b, c];
    vec.sort();
    vec.dedup();
    println!("{}", vec.len());
}