// https://atcoder.jp/contests/arc105/tasks/arc105_b

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ans = 0;
    for v in a {
        ans = gcd(ans, v);
    }
    println!("{}", ans);
}