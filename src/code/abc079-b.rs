// https://atcoder.jp/contests/abc079/tasks/abc079_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut L: Vec<isize> = vec![0; N+1];
    L[0] = 2;
    L[1] = 1;
    for i in 2..=N {
        L[i] = L[i-2] + L[i-1];
    }
    println!("{}", L[N]);
}