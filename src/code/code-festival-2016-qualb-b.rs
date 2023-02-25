// https://atcoder.jp/contests/code-festival-2016-qualb/tasks/codefestival_2016_qualB_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        A: usize,
        B: usize,
        S: String,
    }
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for s in S.chars() {
        if s == 'a' {
            if cnt1 + cnt2 < A + B {
                cnt1 += 1;
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
        else if s == 'b' {
            if cnt1 + cnt2 < A + B && cnt2 < B {
                cnt2 += 1;
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
        else {
            println!("No");
        }
    }
}