// https://atcoder.jp/contests/nomura2020/tasks/nomura2020_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: String,
    }
    println!("{}", T.replace("?", "D"));
}