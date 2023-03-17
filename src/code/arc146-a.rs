// https://atcoder.jp/contests/arc146/tasks/arc146_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    for perm in (0..3).permutations(3) {
        let x = format!("{}{}{}", A[perm[0]], A[perm[1]], A[perm[2]]);
        ans = max(ans, x.parse::<usize>().unwrap());
    }
    println!("{}", ans);
}