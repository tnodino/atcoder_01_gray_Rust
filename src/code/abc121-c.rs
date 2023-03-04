// https://atcoder.jp/contests/abc121/tasks/abc121_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut M: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        vec.push((A, B));
    }
    vec.sort();
    let mut ans = 0;
    for i in 0..N {
        if M <= vec[i].1 {
            ans += vec[i].0 * M;
            break;
        }
        else {
            ans += vec[i].0 * vec[i].1;
            M -= vec[i].1;
        }
    }
    println!("{}", ans);
}