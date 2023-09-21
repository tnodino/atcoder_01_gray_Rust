// https://atcoder.jp/contests/abc320/tasks/abc320_b

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
    for l in (1..=N).rev() {
        'cont: for i in 0..=N-l {
            for j in 0..l {
                if S[i+j] != S[i+l-j-1] {
                    continue 'cont;
                }
            }
            println!("{}", l);
            return;
        }
    }
}