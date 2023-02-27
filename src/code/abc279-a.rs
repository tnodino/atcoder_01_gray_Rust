// https://atcoder.jp/contests/abc279/tasks/abc279_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut ans = 0;
    for s in S.chars() {
        ans += match s {
            'v' => 1,
            'w' => 2,
            _ => unreachable!()
        };
    }
    println!("{}", ans);
}