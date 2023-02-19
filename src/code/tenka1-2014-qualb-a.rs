// https://atcoder.jp/contests/tenka1-2014-qualb/tasks/tenka1_2014_qualB_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", S.replace("HAGIYA", "HAGIXILE"));
}