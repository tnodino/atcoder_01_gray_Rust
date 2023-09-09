// https://atcoder.jp/contests/indeednow-quala/tasks/indeednow_2015_quala_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut T = "indeednow".chars().collect::<Vec<char>>();
    T.sort();
    for _ in 0..N {
        input! {
            S: String,
        }
        let mut S = S.chars().collect::<Vec<char>>();
        S.sort();
        println!("{}", match S == T {
            true => "YES",
            false => "NO",
        })
    }
}