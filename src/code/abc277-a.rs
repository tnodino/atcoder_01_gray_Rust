// https://atcoder.jp/contests/abc277/tasks/abc277_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X): (usize, usize),
        P: [usize; N],
    }
    println!("{}", P.iter().position(|&x| x == X).unwrap() + 1);
}