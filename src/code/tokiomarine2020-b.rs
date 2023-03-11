// https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        V: isize,
        B: isize,
        W: isize,
        T: isize,
    }
    if V <= W {
        println!("NO");
        return;
    }
    let dist = (A - B).abs();
    let time = V - W;
    if (dist + time - 1) / time <= T {
        println!("YES");
    }
    else {
        println!("NO");
    }
}