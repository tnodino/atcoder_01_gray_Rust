// https://atcoder.jp/contests/arc163/tasks/arc163_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        input! {
            N: usize,
            S: String,
        }
        let mut ans = "No";
        for i in 1..N {
            if &S[..i] < &S[i..] {
                ans = "Yes";
            }
        }
        println!("{}", ans);
    }
}