// https://atcoder.jp/contests/abc163/tasks/abc163_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N-1],
    }
    let mut cnt = vec![0; N];
    for a in A {
        cnt[a-1] += 1;
    }
    for c in cnt {
        println!("{}", c);
    }
}