// https://atcoder.jp/contests/abc004/tasks/abc004_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        c: [[String; 4]; 4],
    }
    let mut ans = [["?"; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            ans[i][j] = &c[3-i][3-j];
        }
    }
    for i in 0..4 {
        println!("{}", ans[i].join(" "));
    }
}