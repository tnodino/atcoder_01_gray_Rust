// https://atcoder.jp/contests/code-festival-2017-qualb/tasks/code_festival_2017_qualb_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    println!("{}", &S[0..N-8]);
}