// https://atcoder.jp/contests/arc157/tasks/arc157_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        A: isize,
        B: isize,
        C: isize,
        D: isize,
    }
    if B == 0 && C == 0 {
        if A != 0 && D != 0 {
            println!("No");
        }
        else {
            println!("Yes");
        }
    }
    else {
        if (B - C).abs() <= 1 {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}