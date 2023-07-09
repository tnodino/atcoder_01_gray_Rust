// https://atcoder.jp/contests/abc309/tasks/abc309_b

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
    let mut B = vec![vec!['?'; N]; N];
    for i in 0..N {
        for j in 0..N {
            if 0 < i && j == 0 {
                B[i-1][j] = A[i][j];
            }
            else if i == 0 && j < N - 1 {
                B[i][j+1] = A[i][j];
            }
            else if i < N - 1 && j == N - 1 {
                B[i+1][j] = A[i][j];
            }
            else if i == N - 1 && 0 < j {
                B[i][j-1] = A[i][j];
            }
            else {
                B[i][j] = A[i][j];
            }
        }
    }
    for i in 0..N {
        println!("{}", B[i].iter().map(|&x| x.to_string()).collect::<String>());
    }
}