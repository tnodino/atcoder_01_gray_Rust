// https://atcoder.jp/contests/abc222/tasks/abc222_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
        a: [usize; N],
    }
    let mut ans = 0;
    for v in a {
        if v < P {
            ans += 1;
        }
    }
    println!("{}", ans);
}