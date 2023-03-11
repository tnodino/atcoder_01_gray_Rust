// https://atcoder.jp/contests/abc236/tasks/abc236_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: [String; N],
        T: [String; M],
    }
    let mut idx = 0;
    for s in S {
        if s == T[idx] {
            println!("Yes");
            idx += 1;
        }
        else {
            println!("No");
        }
    }
}