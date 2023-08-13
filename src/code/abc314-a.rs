// https://atcoder.jp/contests/abc314/tasks/abc314_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let S = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".to_string();
    println!("{}", &S[..N+2]);
}