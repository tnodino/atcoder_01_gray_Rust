// https://atcoder.jp/contests/abc297/tasks/abc297_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
        T: [usize; N],
    }
    for i in 0..N-1 {
        if T[i+1] - T[i] <= D {
            println!("{}", T[i+1]);
            return;
        }
    }
    println!("-1");
}