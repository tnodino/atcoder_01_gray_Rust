// https://atcoder.jp/contests/abc283/tasks/abc283_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
    }
    S = S.replace("00", "-");
    println!("{}", S.len());
}