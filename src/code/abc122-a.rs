// https://atcoder.jp/contests/abc122/tasks/abc122_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        b: char,
    }
    println!("{}", match b {
        'A' => 'T',
        'T' => 'A',
        'G' => 'C',
        _ => 'G',
    });
}