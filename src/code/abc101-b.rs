// https://atcoder.jp/contests/abc101/tasks/abc101_b

use proconio::input;
use proconio::fastout;

fn S(mut n: usize) -> usize {
    let mut ret = 0;
    while n > 0 {
        ret += n % 10;
        n /= 10;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N % S(N) == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}