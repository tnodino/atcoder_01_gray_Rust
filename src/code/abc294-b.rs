// https://atcoder.jp/contests/abc294/tasks/abc294_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    for _ in 0..H {
        input! {
            A: [u8; W],
        }
        for a in A {
            print!("{}", match a {
                0 => '.',
                _ => (a + 64) as char,
            });
        }
        println!();
    }
}