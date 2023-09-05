// https://atcoder.jp/contests/abc218/tasks/abc218_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 26;
    input! {
        P: [u8; N],
    }
    let mut ans = vec!['?'; N];
    for i in 0..N {
        ans[i] = ((P[i] - 1) + ('a' as u8)) as char;
    }
    println!("{}", ans.iter().collect::<String>());
}