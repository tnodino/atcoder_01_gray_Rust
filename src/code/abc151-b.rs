// https://atcoder.jp/contests/abc151/tasks/abc151_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        M: usize,
        A: [usize; N-1],
    }
    let sum = A.iter().sum::<usize>();
    if sum + K < N * M {
        println!("-1")
    }
    else if sum >= N * M {
        println!("0");
    }
    else {
        println!("{}", N * M - sum);
    }
}