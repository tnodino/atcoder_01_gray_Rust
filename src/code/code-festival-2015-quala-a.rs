// https://atcoder.jp/contests/code-festival-2015-quala/tasks/codefestival_2015_qualA_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", S.replace("2014", "2015"));
}