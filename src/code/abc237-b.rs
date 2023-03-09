// https://atcoder.jp/contests/abc237/tasks/abc237_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize; W]; H],
    }
    let mut B = vec![vec![0; H]; W];
    for i in 0..H {
        for j in 0..W {
            B[j][i] = A[i][j];
        }
    }
    for i in 0..W {
        println!("{}", B[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}