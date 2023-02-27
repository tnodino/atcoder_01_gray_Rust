// https://atcoder.jp/contests/abc187/tasks/abc187_a

use proconio::input;
use proconio::fastout;

fn sum_digit(mut x: usize) -> usize {
    let mut ret = 0;
    while x > 0 {
        ret += x % 10;
        x /= 10;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if sum_digit(A) >= sum_digit(B) {
        println!("{}", sum_digit(A));
    }
    else {
        println!("{}", sum_digit(B));
    }
}