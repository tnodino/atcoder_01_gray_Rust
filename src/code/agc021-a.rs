// https://atcoder.jp/contests/agc021/tasks/agc021_a

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
    }
    let S = N.to_string();
    let M = format!("{}{}", &S[0..1], "9".repeat(S.len()-1)).parse::<usize>().unwrap();
    if N == M {
        println!("{}", sum_digit(M));
    }
    else {
        println!("{}", sum_digit(M) - 1);
    }
}