// https://atcoder.jp/contests/abc083/tasks/abc083_b

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
        N: usize,
        A: usize,
        B: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        let x = sum_digit(i);
        if A <= x && x <= B {
            ans += i;
        }
    }
    println!("{}", ans);
}