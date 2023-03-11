// https://atcoder.jp/contests/abc261/tasks/abc261_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut ans = "correct";
    for i in 0..N {
        for j in i+1..N {
            if A[i][j] == 'D' {
                if A[i][j] != A[j][i] {
                    ans = "incorrect";
                }
            }
            else {
                if A[i][j] == A[j][i] || A[j][i] == 'D' {
                    ans = "incorrect";
                }
            }
        }
    }
    println!("{}", ans);
}