// https://atcoder.jp/contests/abc254/tasks/abc254_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = vec![1];
    for i in 1..=N {
        println!("{}", A.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        let mut B = Vec::new();
        for j in 0..=i {
            if j == 0 || j == i {
                B.push(1);
            }
            else {
                B.push(A[j-1] + A[j]);
            }
        }
        A = B;
    }
}