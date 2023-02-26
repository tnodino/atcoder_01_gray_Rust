// https://atcoder.jp/contests/abc064/tasks/abc064_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _r: usize,
        g: usize,
        b: usize,
    }
    if (g * 10 + b) % 4 == 0 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}