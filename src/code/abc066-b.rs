// https://atcoder.jp/contests/abc066/tasks/abc066_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    'outer: for i in 1..=N {
        if (N - i) % 2 == 1 {
            continue;
        }
        let n = (N - i) / 2;
        for j in 0..n {
            if S[j] != S[j+n] {
                continue 'outer;
            }
        }
        println!("{}", N - i);
        return;
    }
}