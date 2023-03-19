// https://atcoder.jp/contests/arc147/tasks/arc147_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut mi: usize = !0;
    let mut heap = BinaryHeap::new();
    for a in A {
        heap.push(a);
        mi = min(mi, a);
    }
    let mut ans = 0;
    while heap.len() > 1 {
        ans += 1;
        let h = heap.pop().unwrap();
        if h % mi == 0 {
            continue;
        }
        heap.push(h % mi);
        mi = h % mi;
    }
    println!("{}", ans);
}