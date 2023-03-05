// https://atcoder.jp/contests/abc137/tasks/abc137_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: isize,
        X: isize,
    }
    let vec = (X-K+1..X+K).collect::<Vec<_>>();
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}