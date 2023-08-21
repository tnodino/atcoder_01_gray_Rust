// https://atcoder.jp/contests/abc315/tasks/abc315_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
        D: [usize; M],
    }
    let m = D.iter().sum::<usize>() / 2 + 1;
    let mut s = 0;
    for i in 0..M {
        if s + D[i] >= m {
            println!("{} {}", i + 1, m - s);
            return;
        }
        s += D[i];
    }
}