// https://atcoder.jp/contests/abc154/tasks/abc154_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort();
    A.dedup();
    if A.len() < N {
        println!("NO");
    }
    else {
        println!("YES");
    }
}