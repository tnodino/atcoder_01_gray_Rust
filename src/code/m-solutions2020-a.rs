// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    println!("{}", match X {
        v if v < 600 => 8,
        v if v < 800 => 7,
        v if v < 1000 => 6,
        v if v < 1200 => 5,
        v if v < 1400 => 4,
        v if v < 1600 => 3,
        v if v < 1800 => 2,
        _ => 1,
    })
}