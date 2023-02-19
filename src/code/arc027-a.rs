// https://atcoder.jp/contests/arc027/tasks/arc027_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut h: usize,
        m: usize,
    }
    let mut ans = 60 - m;
    h += 1;
    ans += (18 - h) * 60;
    println!("{}", ans);
}