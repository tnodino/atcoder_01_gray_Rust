// https://atcoder.jp/contests/abc293/tasks/abc293_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    for i in (0..S.len()).step_by(2) {
        S.swap(i, i+1);
    }
    println!("{}", S.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(""));
}