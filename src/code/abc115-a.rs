// https://atcoder.jp/contests/abc115/tasks/abc115_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
    }
    let mut ans = vec!["Christmas", "Eve", "Eve", "Eve"];
    for _ in 0..D-22 {
        ans.pop();
    }
    println!("{}", ans.join(" "));
}