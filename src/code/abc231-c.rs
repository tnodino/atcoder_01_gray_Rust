// https://atcoder.jp/contests/abc231/tasks/abc231_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [usize; N],
    }
    A.sort();
    for _ in 0..Q {
        input! {
            x: usize,
        }
        println!("{}", N - A.binary_search(&x).unwrap_or_else(|x| x));
    }
}