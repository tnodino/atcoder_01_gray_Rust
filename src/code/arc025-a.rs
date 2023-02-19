// https://atcoder.jp/contests/arc025/tasks/arc025_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: [usize; 7],
        J: [usize; 7],
    }
    let mut ans = 0;
    for i in 0..7 {
        ans += D[i].max(J[i]);
    }
    println!("{}", ans);
}