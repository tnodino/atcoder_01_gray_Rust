// https://atcoder.jp/contests/arc031/tasks/arc031_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Name: String,
    }
    let vec: Vec<char> = Name.chars().collect();
    let mut rev_vec = vec.clone();
    rev_vec.reverse();
    let ans;
    if vec == rev_vec {
        ans = "YES";
    }
    else {
        ans = "NO";
    }
    println!("{}", ans);
}