// https://atcoder.jp/contests/abc205/tasks/abc205_c

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
    let A_flg;
    let B_flg;
    if C % 2 == 1 && A < 0 {
        A_flg = true;
    }
    else {
        A_flg = false;
    }
    if C % 2 == 1 && B < 0 {
        B_flg = true;
    }
    else {
        B_flg = false;
    }
    if A_flg != B_flg {
        if A_flg {
            println!("<");
        }
        else {
            println!(">")
        }
    }
    else {
        if A.abs() < B.abs() {
            println!("<");
        }
        else if A.abs() > B.abs() {
            println!(">");
        }
        else {
            println!("=");
        }
    }
}