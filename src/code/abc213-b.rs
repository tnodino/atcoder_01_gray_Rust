// https://atcoder.jp/contests/abc213/tasks/abc213_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut B = A.clone();
    B.sort_by(|a, b| b.cmp(a));
    for i in 0..N {
        if A[i] == B[1] {
            println!("{}", i + 1);
            return;
        }
    }
}