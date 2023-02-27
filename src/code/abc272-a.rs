// https://atcoder.jp/contests/abc272/tasks/abc272_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    println!("{}", A.iter().sum::<usize>());
}