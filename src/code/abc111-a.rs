// https://atcoder.jp/contests/abc111/tasks/abc111_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: String,
    }
    for s in n.chars() {
        print!("{}", match s {
            '1' => '9',
            '9' => '1',
            _ => s,
        });
    }
    println!();
}