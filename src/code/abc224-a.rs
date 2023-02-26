// https://atcoder.jp/contests/abc224/tasks/abc224_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    if &S[N-2..N] == "er" {
        println!("er");
    }
    else {
        println!("ist")
    }
}