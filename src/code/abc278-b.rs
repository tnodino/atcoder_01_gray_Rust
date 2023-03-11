// https://atcoder.jp/contests/abc278/tasks/abc278_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut H: usize,
        mut M: usize,
    }
    loop {
        let a = H / 10;
        let b = H % 10;
        let c = M / 10;
        let d = M % 10;
        let h = a * 10 + c;
        let m = b * 10 + d;
        if h < 24 && m < 60 {
            println!("{} {}", H, M);
            return;
        }
        M += 1;
        if M == 60 {
            H += 1;
            H %= 24;
            M = 0;
        }
    }
}