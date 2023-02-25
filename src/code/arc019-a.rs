// https://atcoder.jp/contests/arc019/tasks/arc019_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    for s in S.chars() {
        print!("{}", match s {
            'O' => '0',
            'D' => '0',
            'I' => '1',
            'Z' => '2',
            'S' => '5',
            'B' => '8',
            _ => s
        });
    }
    println!();
}