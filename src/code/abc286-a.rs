// https://atcoder.jp/contests/abc286/tasks/abc286_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
        Q: usize,
        R: usize,
        _S: usize,
        mut A: [usize; N],
    }
    let M = Q - P + 1;
    for i in 0..M {
        A.swap(P + i - 1, R + i - 1);
    }
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}