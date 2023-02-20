// https://atcoder.jp/contests/abc016/tasks/abc016_2

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
    }
    let ans;
    if A + B == C && A - B == C {
        ans = "?";
    }
    else if A + B == C {
        ans = "+";
    }
    else if A - B == C {
        ans = "-";
    }
    else {
        ans = "!";
    }
    println!("{}", ans);
}