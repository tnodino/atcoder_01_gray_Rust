// https://atcoder.jp/contests/arc021/tasks/arc021_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [[usize; 4]; 4],
    }
    let mut ans = "GAMEOVER";
    for i in 0..4 {
        for j in 0..4 {
            if i <= 2 && A[i][j] == A[i+1][j] {
                ans = "CONTINUE";
            }
            if j <= 2 && A[i][j] == A[i][j+1] {
                ans = "CONTINUE";
            }
        }
    }
    println!("{}", ans);
}