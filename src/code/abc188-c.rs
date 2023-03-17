// https://atcoder.jp/contests/abc188/tasks/abc188_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    for i in 1..=1<<N {
        input! {
            a: usize,
        }
        A.push((a, i));
    }
    while A.len() > 2 {
        let mut B = Vec::new();
        for i in (0..A.len()).step_by(2) {
            if A[i].0 > A[i+1].0 {
                B.push(A[i]);
            }
            else {
                B.push(A[i+1]);
            }
        }
        A = B.clone();
    }
    if A[0].0 > A[1].0 {
        println!("{}", A[1].1);
    }
    else {
        println!("{}", A[0].1);
    }
}