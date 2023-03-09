// https://atcoder.jp/contests/abc249/tasks/abc249_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let mut S = S.chars().collect::<Vec<char>>();
    S.sort();
    S.dedup();
    let mut up = false;
    let mut lo = false;
    for s in S.iter() {
        if s.is_uppercase() {
            up = true;
        }
        else {
            lo = true;
        }
    }
    if S.len() == N && up && lo {
        println!("Yes");
    }
    else {
        println!("No");
    }

}