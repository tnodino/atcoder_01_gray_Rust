// https://atcoder.jp/contests/abc239/tasks/abc239_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    let mut flg = [true; 201];
    for i in 2..=200 {
        for j in 2..i {
            if i % j == 0 {
                flg[i] = false;
                break;
            }
        }
    }
    for i in A..=B {
        let mut chk = true;
        for j in C..=D {
            if flg[i+j] {
                chk = false;
                break;
            }
        }
        if chk {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}