// https://atcoder.jp/contests/keyence2019/tasks/keyence2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N1: usize,
        N2: usize,
        N3: usize,
        N4: usize,
    }
    let mut N = [N1, N2, N3, N4];
    N.sort();
    let mut M = [1, 9, 7, 4];
    M.sort();
    if N == M {
        println!("YES");
    }
    else {
        println!("NO");
    }
}