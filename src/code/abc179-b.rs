// https://atcoder.jp/contests/abc179/tasks/abc179_b

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
            D1: usize,
            D2: usize,
        }
        if D1 == D2 {
            cnt += 1;
            if cnt == 3 {
                println!("Yes");
                return;
            }
        }
        else {
            cnt = 0;
        }
    }
    println!("No");
}