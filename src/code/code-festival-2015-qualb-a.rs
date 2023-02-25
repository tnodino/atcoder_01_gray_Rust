// https://atcoder.jp/contests/code-festival-2015-qualb/tasks/codefestival_2015_qualB_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}{}", S, S);
}