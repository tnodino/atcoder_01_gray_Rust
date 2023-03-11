// https://atcoder.jp/contests/abc185/tasks/abc185_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
    }
    let mut pascal: Vec<Vec<usize>> = vec![vec![0; 12]; L];
    pascal[0][0] = 1;
    for n in 1..L {
        pascal[n][0] = 1;
        for k in 1..=n {
            if k >= 12 {
                break;
            }
            pascal[n][k] = pascal[n-1][k-1] + pascal[n-1][k];
        }
    }
    println!("{}", pascal[L-1][11]);
}