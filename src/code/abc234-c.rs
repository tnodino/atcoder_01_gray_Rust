// https://atcoder.jp/contests/abc234/tasks/abc234_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut K: usize,
    }
    let mut ans = Vec::new();
    while K > 0 {
        if K & 1 == 0 {
            ans.push(0);
        }
        else {
            ans.push(2);
        }
        K /= 2;
    }
    ans.reverse();
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""));
}