// https://atcoder.jp/contests/abc189/tasks/abc189_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        C: String,
    }
    let C = C.chars().collect::<Vec<char>>();
    println!("{}", match (C[0] == C[1], C[1] == C[2]) {
        (true, true) => "Won",
        (_, _) => "Lost",
    })
}