// https://atcoder.jp/contests/arc049/tasks/arc049_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (S, A, B, C, D): (String, usize, usize, usize, usize),
    }
    println!("{}\"{}\"{}\"{}\"{}", &S[..A], &S[A..B], &S[B..C], &S[C..D], &S[D..]);
}