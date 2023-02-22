// https://atcoder.jp/contests/abc019/tasks/abc019_1

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
    println!("{}", vec[1]);
}