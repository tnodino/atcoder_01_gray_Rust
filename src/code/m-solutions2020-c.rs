// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    for i in K..N {
        if A[i-K] < A[i] {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}