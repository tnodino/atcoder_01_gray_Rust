// https://atcoder.jp/contests/abc292/tasks/abc292_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", S.to_uppercase());
}