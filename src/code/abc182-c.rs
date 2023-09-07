// https://atcoder.jp/contests/abc182/tasks/abc182_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    let M = N.len();
    let N = N.chars().collect::<Vec<char>>();
    let mut ans = M;
    for bit in 0..1<<M {
        let mut num = 0;
        let mut cnt = 0;
        for i in 0..M {
            if bit & (1 << i) > 0 {
                num *= 10;
                num += (N[i] as usize) - ('0' as usize);
            }
            else {
                cnt += 1;
            }
        }
        if num % 3 == 0 {
            ans = min(ans, cnt);
        }
    }
    match ans == M {
        true => println!("-1"),
        false => println!("{}", ans),
    }
}