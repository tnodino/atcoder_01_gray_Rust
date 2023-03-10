// https://atcoder.jp/contests/jsc2021/tasks/jsc2021_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let A = A.iter().collect::<HashSet<_>>();
    let B = B.iter().collect::<HashSet<_>>();
    let ans = A.symmetric_difference(&B);
    let mut ans = ans.into_iter().collect::<Vec<_>>();
    ans.sort();
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}