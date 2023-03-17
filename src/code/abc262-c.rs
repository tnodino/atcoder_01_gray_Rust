// https://atcoder.jp/contests/abc262/tasks/abc262_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut cnt1: usize = 0;
    let mut cnt2: usize = 0;
    for i in 0..N {
        if a[i] == i + 1 {
            cnt1 += 1;
        }
        else if a[a[i]-1] == i + 1 {
            cnt2 += 1;
        }
    }
    println!("{}", (cnt1 - 1) * cnt1 / 2 + cnt2 / 2);
}