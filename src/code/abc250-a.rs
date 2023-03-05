// https://atcoder.jp/contests/abc250/tasks/abc250_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        R: usize,
        C: usize,
    }
    let mut ans = 4;
    if R == 1 {
        ans -= 1;
    }
    if R == H {
        ans -= 1;
    }
    if C == 1 {
        ans -= 1;
    }
    if C == W {
        ans -= 1;
    }
    println!("{}", ans);
}