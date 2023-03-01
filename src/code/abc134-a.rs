// https://atcoder.jp/contests/abc134/tasks/abc134_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        r: usize,
    }
    println!("{}", 3 * r * r);
}