// https://atcoder.jp/contests/abc287/tasks/abc287_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: [String; N],
        T: [String; M],
    }
    let mut ans = 0;
    for i in 0..N {
        if T.contains(&S[i][3..].to_string()) {
            ans += 1;
        }
    }
    println!("{}", ans);
}