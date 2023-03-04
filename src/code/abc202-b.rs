// https://atcoder.jp/contests/abc202/tasks/abc202_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    for s in S.chars().rev() {
        print!("{}", match s {
            '6' => '9',
            '9' => '6',
            _ => s,
        });
    }
    println!();
}