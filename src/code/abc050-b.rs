// https://atcoder.jp/contests/abc050/tasks/abc050_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: [usize; N],
    }
    let mut time = 0;
    for i in 0..N {
        time += T[i];
    }
    input! {
        M: usize,
    }
    for _ in 0..M {
        input! {
            P: usize,
            X: usize,
        }
        println!("{}", time - T[P-1] + X);
    }
}