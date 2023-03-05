// https://atcoder.jp/contests/abc280/tasks/abc280_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [isize; N],
    }
    let mut A = vec![0; N];
    A[0] = S[0];
    for i in 1..N {
        A[i] = S[i] - S[i-1];
    }
    println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}