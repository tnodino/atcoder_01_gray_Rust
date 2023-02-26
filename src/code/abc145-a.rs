// https://atcoder.jp/contests/abc145/tasks/abc145_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        r: usize,
    }
    println!("{}", r * r);
}