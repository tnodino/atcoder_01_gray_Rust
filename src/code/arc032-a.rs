// https://atcoder.jp/contests/arc032/tasks/arc032_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let N = (n + 1) * n / 2;
    if N == 1 {
        println!("BOWWOW");
        return;
    }
    for i in 2..N {
        if N % i == 0 {
            println!("BOWWOW");
            return;
        }
    }
    println!("WANWAN");
}