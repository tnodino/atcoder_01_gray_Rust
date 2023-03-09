// https://atcoder.jp/contests/abc153/tasks/abc153_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut H: [usize; N],
    }
    H.sort_by(|a, b| b.cmp(a));
    let mut ans: usize = 0;
    for i in K..N {
        ans += H[i];
    }
    println!("{}", ans);
}