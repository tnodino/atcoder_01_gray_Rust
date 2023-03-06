// https://atcoder.jp/contests/abc268/tasks/abc268_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    if N > T.len() {
        println!("No");
    }
    else if S == &T[..N] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}