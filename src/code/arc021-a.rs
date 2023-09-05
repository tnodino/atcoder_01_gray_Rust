// https://atcoder.jp/contests/arc021/tasks/arc021_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 4;
    input! {
        A: [[usize; N]; N],
    }
    for i in 0..N {
        for j in 0..N {
            if i + 1 < N && A[i][j] == A[i+1][j] {
                println!("CONTINUE");
                return;
            }
            if j + 1 < N && A[i][j] == A[i][j+1] {
                println!("CONTINUE");
                return;
            }
        }
    }
    println!("GAMEOVER");
}