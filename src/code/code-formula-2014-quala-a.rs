// https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
    }
    let mut ans = "NO";
    for a in 1..=A {
        if a * a * a == A {
            ans = "YES";
        }
    }
    println!("{}", ans);
}