// https://atcoder.jp/contests/abc256/tasks/abc256_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut P = 0;
    let mut cnt = [0; 4];
    for a in A {
        cnt[0] += 1;
        for i in (0..4).rev() {
            if i + a >= 4 {
                P += cnt[i];
                cnt[i] = 0;
            }
            else {
                cnt[i+a] += cnt[i];
                cnt[i] = 0;
            }
        }
    }
    println!("{}", P);
}