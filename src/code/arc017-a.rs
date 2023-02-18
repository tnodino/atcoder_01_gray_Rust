// https://atcoder.jp/contests/arc017/tasks/arc017_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }
    let mut ans = "YES";
    for i in 2..N {
        if N % i == 0 {
            ans = "NO";
        }
    }
    println!("{}", ans);
}