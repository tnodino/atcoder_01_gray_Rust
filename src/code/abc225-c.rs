// https://atcoder.jp/contests/abc225/tasks/abc225_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        B: [[usize; M]; N],
    }
    let mo = (B[0][0] - 1) % 7;
    if mo + M > 7 {
        println!("No");
        return;
    }
    for i in 0..N {
        for j in 0..M {
            if i + 1 < N && B[i][j] + 7 != B[i+1][j] {
                println!("No");
                return;
            }
            if j + 1 < M && B[i][j] + 1 != B[i][j+1] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}