// https://atcoder.jp/contests/abc265/tasks/abc265_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut T: usize,
        A: [usize; N-1],
    }
    let mut bonus = vec![0; N];
    for _ in 0..M {
        input! {
            X: usize,
            Y: usize,
        }
        bonus[X-1] += Y;
    }
    for i in 0..N-1 {
        if A[i] >= T {
            println!("No");
            return;
        }
        T -= A[i];
        T += bonus[i+1];
    }
    println!("Yes");
}