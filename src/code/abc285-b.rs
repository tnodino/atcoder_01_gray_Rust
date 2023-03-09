// https://atcoder.jp/contests/abc285/tasks/abc285_b

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
    for x in 1..N {
        let mut cnt = 0;
        for i in 0..N-x {
            if S[i] == S[i+x] {
                break;
            }
            cnt += 1;
        }
        println!("{}", cnt);
    }
}