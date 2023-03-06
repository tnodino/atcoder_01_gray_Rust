// https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    let mut ans = 0;
    ans += match X {
        1 => 300000,
        2 => 200000,
        3 => 100000,
        _ => 0,
    };
    ans += match Y {
        1 => 300000,
        2 => 200000,
        3 => 100000,
        _ => 0,
    };
    if X == 1 && Y == 1 {
        ans += 400000;
    }
    println!("{}", ans);
}