// https://atcoder.jp/contests/nikkei2019-2-qual/tasks/nikkei2019_2_qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N - 1) / 2);
}