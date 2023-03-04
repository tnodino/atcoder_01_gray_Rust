// https://atcoder.jp/contests/abc278/tasks/abc278_a

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [usize; N],
    }
    let mut deq = VecDeque::new();
    for i in 0..N {
        deq.push_back(A[i]);
    }
    for _ in 0..K {
        deq.pop_front();
        deq.push_back(0);
    }
    println!("{}", deq.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}