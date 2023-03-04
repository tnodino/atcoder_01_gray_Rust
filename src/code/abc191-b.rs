// https://atcoder.jp/contests/abc191/tasks/abc191_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }
    let A = A.iter().filter(|&x| x != &X).collect::<Vec<_>>();
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}