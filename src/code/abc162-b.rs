// https://atcoder.jp/contests/abc162/tasks/abc162_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        if i % 3 != 0 && i % 5 != 0 {
            ans += i;
        }
    }
    println!("{}", ans);
}