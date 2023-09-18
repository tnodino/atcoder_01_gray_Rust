// https://atcoder.jp/contests/arc046/tasks/arc046_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 1_000_000;
    let mut vec = Vec::new();
    for i in 1..10 {
        let mut x = i;
        while x < M {
            vec.push(x);
            x = x * 10 + i;
        }
    }
    vec.sort();
    println!("{}", vec[N-1]);
}