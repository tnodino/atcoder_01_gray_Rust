// https://atcoder.jp/contests/abc195/tasks/abc195_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
        H: usize,
    }
    if H % M == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}