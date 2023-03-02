// https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", &S[..3]);
}