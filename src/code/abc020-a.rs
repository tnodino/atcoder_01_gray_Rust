// https://atcoder.jp/contests/abc020/tasks/abc020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    println!("{}", match Q {
        1 => "ABC",
        2 => "chokudai",
        _ => unreachable!()
    });
}