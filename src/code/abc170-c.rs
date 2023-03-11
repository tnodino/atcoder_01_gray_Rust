// https://atcoder.jp/contests/abc170/tasks/abc170_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        N: usize,
        p: [usize; N],
    }
    for i in 0..=N {
        if !p.contains(&(X-i)) {
            println!("{}", X - i);
            return;
        }
        if !p.contains(&(X+i)) {
            println!("{}", X + i);
            return;
        }
    }
}