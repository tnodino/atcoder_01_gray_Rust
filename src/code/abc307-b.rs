// https://atcoder.jp/contests/abc307/tasks/abc307_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }
    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }
            let s = format!("{}{}", S[i], S[j]);
            let t = s.chars().rev().map(|x| x.to_string()).collect::<String>();
            if s == t {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}