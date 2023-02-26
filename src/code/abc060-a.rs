// https://atcoder.jp/contests/abc060/tasks/abc060_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
        B: String,
        C: String,
    }
    let S = [A, B, C];
    for i in 0..2 {
        let x = S[i].chars().last().unwrap();
        let y = S[i+1].chars().next().unwrap();
        if x != y {
            println!("NO");
            return;
        }
    }
    println!("YES");
}