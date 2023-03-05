// https://atcoder.jp/contests/abc171/tasks/abc171_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut p: [usize; N],
    }
    p.sort();
    println!("{}", p[..K].iter().sum::<usize>());
}