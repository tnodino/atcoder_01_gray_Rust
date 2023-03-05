// https://atcoder.jp/contests/abc132/tasks/abc132_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut T = S.chars().collect::<Vec<char>>();
    T.sort();
    T.dedup();
    if T.len() != 2 {
        println!("No");
    }
    else if S.chars().filter(|&x| x == T[0]).count() != 2 {
        println!("No");
    }
    else {
        println!("Yes");
    }
}