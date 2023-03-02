// https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: char,
        T: char,
    }
    if S == 'Y' {
        println!("{}", T.to_uppercase());
    }
    else {
        println!("{}", T);
    }
}