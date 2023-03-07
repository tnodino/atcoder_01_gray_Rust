// https://atcoder.jp/contests/abc244/tasks/abc244_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        T: String,
    }
    let mut x = 0;
    let mut y = 0;
    let mut flg = 0;
    for t in T.chars() {
        match (t, flg) {
            ('S', 0) => x += 1,
            ('S', 1) => y -= 1,
            ('S', 2) => x -= 1,
            ('S', 3) => y += 1,
            (_, _) => {
                flg += 1;
                flg %= 4;
            }
        }
    }
    println!("{} {}", x, y);
}