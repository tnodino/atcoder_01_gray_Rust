// https://atcoder.jp/contests/abc312/tasks/abc312_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let vec = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];
    if vec.contains(&(S.as_str())) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}