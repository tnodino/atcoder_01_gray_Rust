// https://atcoder.jp/contests/abc116/tasks/abc116_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        AB: usize,
        BC: usize,
        _CA: usize,
    }
    println!("{}", AB * BC / 2);
}