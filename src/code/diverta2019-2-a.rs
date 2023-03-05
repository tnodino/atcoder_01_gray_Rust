// https://atcoder.jp/contests/diverta2019-2/tasks/diverta2019_2_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    if K == 1 {
        println!("0");
    }
    else {
        println!("{}", N - K);
    }
}