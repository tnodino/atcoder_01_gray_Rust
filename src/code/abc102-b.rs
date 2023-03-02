// https://atcoder.jp/contests/abc102/tasks/abc102_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let ma = A.iter().max().unwrap();
    let mi = A.iter().min().unwrap();
    println!("{}", ma - mi);
}