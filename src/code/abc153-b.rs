// https://atcoder.jp/contests/abc153/tasks/abc153_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        N: usize,
        A: [usize; N],
    }
    if A.iter().sum::<usize>() >= H {
        println!("Yes");
    }
    else {
        println!("No");
    }
}