// https://atcoder.jp/contests/abc132/tasks/abc132_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut d: [usize; N],
    }
    d.sort();
    println!("{}", d[N/2] - d[N/2-1]);
}