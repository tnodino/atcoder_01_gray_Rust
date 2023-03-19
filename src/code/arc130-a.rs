// https://atcoder.jp/contests/arc130/tasks/arc130_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut cnt: usize = 1;
    let mut ans: usize = 0;
    for i in 1..N {
        if S[i-1] != S[i] {
            ans += (cnt - 1) * cnt / 2;
            cnt = 1;
        }
        else {
            cnt += 1;
        }
    }
    println!("{}", ans + (cnt - 1) * cnt / 2);
}