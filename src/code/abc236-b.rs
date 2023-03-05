// https://atcoder.jp/contests/abc236/tasks/abc236_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N*4-1],
    }
    let mut cnt = vec![0; N+1];
    for a in A.iter() {
        cnt[*a] += 1;
    }
    for i in 1..=N {
        if cnt[i] == 3 {
            println!("{}", i);
            return;
        }
    }
}