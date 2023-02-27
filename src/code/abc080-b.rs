// https://atcoder.jp/contests/abc080/tasks/abc080_b

use proconio::input;
use proconio::fastout;

fn f(mut x: usize) -> usize {
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
        N: usize,
    }
    if N % f(N) == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}