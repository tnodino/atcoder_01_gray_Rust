// https://atcoder.jp/contests/agc010/tasks/agc010_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut cnt = 0;
    for i in 0..N {
        cnt += A[i] % 2;
    }
    println!("{}", match cnt % 2 {
        1 => "NO",
        _ => "YES"
    });
}