// https://atcoder.jp/contests/abc286/tasks/abc286_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    println!("{}", S.replace("na", "nya"));
}