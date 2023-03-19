// https://atcoder.jp/contests/abc216/tasks/abc216_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut ans = Vec::new();
    while N > 0 {
        if N % 2 == 1 {
            ans.push('A');
        }
        ans.push('B');
        N /= 2;
    }
    ans.reverse();
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""));
}