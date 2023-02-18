// https://atcoder.jp/contests/tenka1-2013-quala/tasks/tenka1_2013_qualA_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut ans = 42;
    while ans < 130_000_000 {
        ans *= 2;
    }
    println!("{}", ans);
}