// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for i in 1..=N {
        if i * 108 / 100 == N {
            println!("{}", i);
            return;
        }
    }
    println!(":(");
}