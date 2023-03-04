// https://atcoder.jp/contests/abc243/tasks/abc243_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut V: isize,
        A: isize,
        B: isize,
        C: isize,
    }
    let mut flg = 0;
    while V >= 0 {
        V -= match flg {
            0 => A,
            1 => B,
            _ => C,
        };
        if V >= 0 {
            flg += 1;
            flg %= 3;
        }
    }
    println!("{}", match flg {
        0 => "F",
        1 => "M",
        _ => "T",
    });
}