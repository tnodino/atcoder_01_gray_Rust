// https://atcoder.jp/contests/abc039/tasks/abc039_b

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    for i in 1..=X {
        if pow(i, 4) == X {
            println!("{}", i);
            break;
        }
    }
}