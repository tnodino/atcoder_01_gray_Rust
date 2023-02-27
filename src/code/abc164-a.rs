// https://atcoder.jp/contests/abc164/tasks/abc164_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize,
        W: usize,
    }
    if S <= W {
        println!("unsafe");
    }
    else {
        println!("safe");
    }
}