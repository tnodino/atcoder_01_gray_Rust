// https://atcoder.jp/contests/abc254/tasks/abc254_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    println!("{}", &N[1..]);
}