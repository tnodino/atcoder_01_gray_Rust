// https://atcoder.jp/contests/tenka1-2013-quala/tasks/tenka1_2013_qualA_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            V: usize,
            W: usize,
            X: usize,
            Y: usize,
            Z: usize,
        }
        if V + W + X + Y + Z < 20 {
            ans += 1;
        }
    }
    println!("{}", ans);
}