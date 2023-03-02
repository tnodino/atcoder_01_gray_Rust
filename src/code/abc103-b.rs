// https://atcoder.jp/contests/abc103/tasks/abc103_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: String,
        T: String,
    }
    let N = S.len();
    for _ in 0..N {
        if S == T {
            println!("Yes");
            return;
        }
        S = format!("{}{}", &S[1..], &S[..1]);
    }
    println!("No");
}