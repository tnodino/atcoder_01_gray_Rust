// https://atcoder.jp/contests/soundhound2018-summer-qual/tasks/soundhound2018_summer_qual_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        w: usize,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in 0..S.len() {
        if i % w == 0 {
            print!("{}", S[i]);
        }
    }
    println!();
}