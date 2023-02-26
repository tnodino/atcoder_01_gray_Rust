// https://atcoder.jp/contests/abc067/tasks/abc067_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut l: [usize; N],
    }
    l.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    for i in 0..K {
        ans += l[i];
    }
    println!("{}", ans);
}