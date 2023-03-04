// https://atcoder.jp/contests/abc188/tasks/abc188_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
        B: [isize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        ans += A[i] * B[i];
    }
    if ans == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}