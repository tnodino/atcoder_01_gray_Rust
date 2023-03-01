// https://atcoder.jp/contests/abc287/tasks/abc287_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = 0;
    for _ in 0..N {
        input! {
            S: String,
        }
        if S == "For" {
            cnt += 1;
        }
    }
    if (N + 1) / 2 <= cnt {
        println!("Yes");
    }
    else {
        println!("No");
    }
}