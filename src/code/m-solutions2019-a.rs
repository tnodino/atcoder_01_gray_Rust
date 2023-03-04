// https://atcoder.jp/contests/m-solutions2019/tasks/m_solutions2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", (N - 2) * 180);
}