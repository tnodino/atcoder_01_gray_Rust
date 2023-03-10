// https://atcoder.jp/contests/abc161/tasks/abc161_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [f64; N],
    }
    A.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let sum = A.iter().sum::<f64>();
    let limit = sum / (M as f64 * 4.);
    if A[M-1] < limit {
        println!("No");
    }
    else {
        println!("Yes");
    }
}