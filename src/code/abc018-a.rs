// https://atcoder.jp/contests/abc018/tasks/abc018_1

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

fn rank(x: usize, ma: usize, mi: usize) -> usize {
    if x == ma {
        return 1;
    }
    if x == mi {
        return 3;
    }
    return 2;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let ma = max(A, max(B, C));
    let mi = min(A, min(B, C));
    let array = [A, B, C];
    for i in 0..3 {
        println!("{}", rank(array[i], ma, mi));
    }
}