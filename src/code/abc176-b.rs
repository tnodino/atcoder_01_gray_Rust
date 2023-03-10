// https://atcoder.jp/contests/abc176/tasks/abc176_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    let N = N.chars().collect::<Vec<char>>();
    let mut mo = 0;
    for n in N {
        mo += n as u8 - 48;
        mo %= 9;
    }
    if mo == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}