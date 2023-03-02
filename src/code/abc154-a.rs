// https://atcoder.jp/contests/abc154/tasks/abc154_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        _T: String,
        A: usize,
        B: usize,
        U: String,
    }
    if S == U {
        println!("{} {} ", A - 1, B);
    }
    else {
        println!("{} {} ", A, B - 1);
    }
}