// https://atcoder.jp/contests/abc073/tasks/abc073_b

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
            l: usize,
            r: usize,
        }
        ans += r - l + 1;
    }
    println!("{}", ans);
}