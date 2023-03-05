// https://atcoder.jp/contests/abc229/tasks/abc229_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S1: String,
        S2: String,
    }
    let S1 = S1.chars().collect::<Vec<char>>();
    let S2 = S2.chars().collect::<Vec<char>>();
    if S1[0] == '#' && S1[1] == '.' && S2[0] == '.' && S2[1] == '#' {
        println!("No");
    }
    else if S1[0] == '.' && S1[1] == '#' && S2[0] == '#' && S2[1] == '.' {
        println!("No");
    }
    else {
        println!("Yes");
    }
}