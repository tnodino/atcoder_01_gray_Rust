// https://atcoder.jp/contests/abc274/tasks/abc274_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut cnt = vec![0; W];
    for _ in 0..H {
        input! {
            C: String,
        }
        let C = C.chars().collect::<Vec<char>>();
        for j in 0..W {
            if C[j] == '#' {
                cnt[j] += 1;
            }
        }
    }
    println!("{}", cnt.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}