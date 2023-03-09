// https://atcoder.jp/contests/abc288/tasks/abc288_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        K: usize,
        mut S: [String; K],
    }
    S.sort();
    for s in S {
        println!("{}", s);
    }
}