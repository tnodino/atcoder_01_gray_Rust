// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }
    for i in K..N {
        println!("{}", match A[i] > A[i-K] {
            true => "Yes",
            false => "No",
        });
    }
}