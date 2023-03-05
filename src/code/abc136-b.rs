// https://atcoder.jp/contests/abc136/tasks/abc136_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        let M = i.to_string();
        if M.len() % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}