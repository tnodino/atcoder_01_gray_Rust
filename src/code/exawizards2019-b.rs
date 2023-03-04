// https://atcoder.jp/contests/exawizards2019/tasks/exawizards2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        s: String,
    }
    let red = s.chars().filter(|&x| x == 'R').count();
    let blue = N - red;
    if red > blue {
        println!("Yes");
    }
    else {
        println!("No");
    }
}