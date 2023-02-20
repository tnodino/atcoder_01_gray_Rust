// https://atcoder.jp/contests/abc018/tasks/abc018_1

use proconio::input;
use proconio::fastout;

fn rank(x: usize, ma: usize, mi: usize) -> usize {
    let ret;
    if x == ma {
        ret = 1;
    }
    else if x == mi {
        ret = 3;
    }
    else {
        ret = 2;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let ma = A.max(B.max(C));
    let mi = A.min(B.min(C));
    let vec = vec![A, B, C];
    for i in 0..3 {
        println!("{}", rank(vec[i], ma, mi));
    }
}