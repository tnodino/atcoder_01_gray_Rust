// https://atcoder.jp/contests/arc136/tasks/arc136_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        mut S: String,
    }
    S = S.replace("A", "BB");
    S = S.replace("BB", "A");
    println!("{}",  S);
}