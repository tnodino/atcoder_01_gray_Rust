// https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    if H == 1 || W == 1 {
        println!("1");
    }
    else {
        println!("{}", (H * W + 1) / 2);
    }
}