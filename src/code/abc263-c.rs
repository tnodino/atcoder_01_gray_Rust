// https://atcoder.jp/contests/abc263/tasks/abc263_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    for perm in (1..=M).combinations(N) {
        println!("{}", perm.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}