// https://atcoder.jp/contests/code-festival-2014-qualb/tasks/code_festival_qualB_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut cnt = 0;
    for i in 1..=N {
        input! {
            a: usize,
        }
        cnt += a;
        if cnt >= K {
            println!("{}", i);
            return;
        }
    }
}