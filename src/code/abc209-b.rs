// https://atcoder.jp/contests/abc209/tasks/abc209_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }
    let S = A.iter().sum::<usize>() - N / 2;
    if S <= X {
        println!("Yes");
    }
    else {
        println!("No");
    }
}