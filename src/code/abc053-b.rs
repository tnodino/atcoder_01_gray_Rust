// https://atcoder.jp/contests/abc053/tasks/abc053_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let s = s.chars().collect::<Vec<char>>();
    let N = s.len();
    let mut a = 0;
    for i in 0..N {
        if s[i] == 'A' {
            a = i;
            break;
        }
    }
    let mut z = 0;
    for i in (0..N).rev() {
        if s[i] == 'Z' {
            z = i;
            break;
        }
    }
    println!("{}", z - a + 1);
}