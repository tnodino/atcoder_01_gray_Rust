// https://atcoder.jp/contests/abc085/tasks/abc085_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut d: [usize; N],
    }
    d.sort();
    let mut top = 0;
    let mut ans = 0;
    for i in 0..N {
        if top < d[i] {
            top = d[i];
            ans += 1;
        }
    }
    println!("{}", ans);
}